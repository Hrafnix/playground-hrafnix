use crate::component::ComponentCapability;
use crate::diagnostic::{Diagnostic, DiagnosticCategory, DiagnosticSeverity, EntityReference};
use crate::identity::{ComponentId, SystemId};
use crate::resolve::{ResolvedComponent, ResolvedComponentSource, ResolvedModel, ResolvedSystem};
use std::collections::{BTreeMap, BTreeSet};

/// Deterministic execution order for one system and its nested custom components.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SystemSchedule {
    /// Stable source-system identity.
    pub system_id: SystemId,
    /// Component order for current-step output propagation.
    pub component_order: Vec<ComponentId>,
    /// Private-system plans keyed by their owning custom-component instance.
    pub nested: BTreeMap<ComponentId, SystemSchedule>,
}

/// Direct-feedthrough scheduling failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScheduleFailure {
    /// Navigable validation diagnostic.
    pub diagnostic: Diagnostic,
    /// Complete repeated-endpoint component path through the direct loop.
    pub component_path: Vec<ComponentId>,
}

/// Builds a deterministic recursive execution plan.
///
/// # Errors
///
/// Returns a validation failure when a system contains a direct-feedthrough loop.
pub fn build_schedule(model: &ResolvedModel) -> Result<SystemSchedule, ScheduleFailure> {
    schedule_system(&model.root)
}

/// Builds the plan for one resolved system and all nested private systems.
fn schedule_system(system: &ResolvedSystem) -> Result<SystemSchedule, ScheduleFailure> {
    let components: BTreeMap<ComponentId, &ResolvedComponent> = system
        .components
        .iter()
        .map(|component| (component.id, component))
        .collect();
    let mut outgoing: BTreeMap<ComponentId, BTreeSet<ComponentId>> = components
        .keys()
        .copied()
        .map(|id| (id, BTreeSet::new()))
        .collect();
    let mut incoming_count: BTreeMap<ComponentId, usize> =
        components.keys().copied().map(|id| (id, 0)).collect();

    for connection in &system.connections {
        let Some(target) = components.get(&connection.target.component_id) else {
            continue;
        };
        if !is_direct_feedthrough(target)
            || !components.contains_key(&connection.source.component_id)
        {
            continue;
        }
        let Some(targets) = outgoing.get_mut(&connection.source.component_id) else {
            continue;
        };
        if targets.insert(connection.target.component_id) {
            let Some(count) = incoming_count.get_mut(&connection.target.component_id) else {
                continue;
            };
            *count = count.saturating_add(1);
        }
    }

    let mut ready: BTreeSet<ComponentId> = incoming_count
        .iter()
        .filter_map(|(id, count)| (*count == 0).then_some(*id))
        .collect();
    let mut component_order = Vec::with_capacity(components.len());
    while let Some(id) = ready.pop_first() {
        component_order.push(id);
        let Some(targets) = outgoing.get(&id) else {
            continue;
        };
        for target in targets {
            let Some(count) = incoming_count.get_mut(target) else {
                continue;
            };
            *count = count.saturating_sub(1);
            if *count == 0 {
                ready.insert(*target);
            }
        }
    }

    if component_order.len() != components.len() {
        let blocked: BTreeSet<ComponentId> = incoming_count
            .iter()
            .filter_map(|(id, count)| (*count > 0).then_some(*id))
            .collect();
        let component_path = find_cycle(&outgoing, &blocked);
        let component_id = component_path
            .first()
            .copied()
            .or_else(|| blocked.first().copied());
        if let Some(component_id) = component_id {
            return Err(ScheduleFailure {
                diagnostic: Diagnostic::new(
                    DiagnosticSeverity::Error,
                    DiagnosticCategory::Validation,
                    Some(EntityReference::Component(component_id)),
                    Some("connections".into()),
                    "simulation_schedule_direct_feedthrough_loop",
                ),
                component_path,
            });
        }
    }

    let mut nested = BTreeMap::new();
    for component in components.values() {
        if let ResolvedComponentSource::Custom { implementation, .. } = &component.source {
            nested.insert(component.id, schedule_system(implementation)?);
        }
    }

    Ok(SystemSchedule {
        system_id: system.id,
        component_order,
        nested,
    })
}

/// Returns whether current outputs depend on current inputs.
fn is_direct_feedthrough(component: &ResolvedComponent) -> bool {
    component
        .capabilities
        .contains(ComponentCapability::DirectFeedthrough)
}

/// Finds the first stable-ID-ordered cycle among blocked components.
fn find_cycle(
    outgoing: &BTreeMap<ComponentId, BTreeSet<ComponentId>>,
    blocked: &BTreeSet<ComponentId>,
) -> Vec<ComponentId> {
    let mut states = BTreeMap::<ComponentId, VisitState>::new();
    let mut path = Vec::new();
    for id in blocked {
        if !states.contains_key(id) {
            if let Some(cycle) = visit(*id, outgoing, blocked, &mut states, &mut path) {
                return cycle;
            }
        }
    }
    Vec::new()
}

/// Depth-first search state for deterministic cycle extraction.
#[derive(Clone, Copy, PartialEq, Eq)]
enum VisitState {
    /// Node is on the current search path.
    Visiting,
    /// Node and all outgoing dependencies have been explored.
    Complete,
}

/// Visits one dependency subtree and returns its first repeated-endpoint cycle.
fn visit(
    id: ComponentId,
    outgoing: &BTreeMap<ComponentId, BTreeSet<ComponentId>>,
    blocked: &BTreeSet<ComponentId>,
    states: &mut BTreeMap<ComponentId, VisitState>,
    path: &mut Vec<ComponentId>,
) -> Option<Vec<ComponentId>> {
    states.insert(id, VisitState::Visiting);
    path.push(id);
    if let Some(targets) = outgoing.get(&id) {
        for target in targets.iter().filter(|target| blocked.contains(target)) {
            match states.get(target) {
                Some(VisitState::Visiting) => {
                    let start = path.iter().position(|item| item == target).unwrap_or(0);
                    let mut cycle = path.get(start..).unwrap_or_default().to_vec();
                    cycle.push(*target);
                    return Some(cycle);
                }
                Some(VisitState::Complete) => {}
                None => {
                    if let Some(cycle) = visit(*target, outgoing, blocked, states, path) {
                        return Some(cycle);
                    }
                }
            }
        }
    }
    path.pop();
    states.insert(id, VisitState::Complete);
    None
}

#[cfg(test)]
mod tests {
    use super::build_schedule;
    use crate::component::{
        ComponentCapabilities, ComponentCapability, ComponentTypeId, SemanticVersion,
    };
    use crate::document::{Connection, LoggingPolicy, PortEndpoint, SimulationSettings};
    use crate::identity::{ComponentId, ConnectionId, DocumentId, SystemId};
    use crate::resolve::{
        ResolvedComponent, ResolvedComponentSource, ResolvedModel, ResolvedSystem, SourceProvenance,
    };
    use crate::timing::FixedStepSemantics;
    use std::collections::BTreeMap;

    fn component(id: u128, direct_feedthrough: bool) -> ResolvedComponent {
        let id = ComponentId::from_raw(id);
        let capabilities = if direct_feedthrough {
            ComponentCapabilities::new([ComponentCapability::DirectFeedthrough])
        } else {
            ComponentCapabilities::default()
        };
        ResolvedComponent {
            id,
            name: format!("component-{id}").into(),
            parameters: vec![],
            ports: vec![],
            public_port_ids: BTreeMap::new(),
            capabilities,
            parameter_overrides: BTreeMap::new(),
            enabled: true,
            source: ResolvedComponentSource::BuiltIn {
                type_id: ComponentTypeId::new("test.component").unwrap(),
                version: SemanticVersion {
                    major: 1,
                    minor: 0,
                    patch: 0,
                },
            },
            provenance: SourceProvenance {
                document_id: DocumentId::from_raw(1),
                system_id: SystemId::from_raw(10),
                component_id: id,
            },
        }
    }

    fn connection(id: u128, source: u128, target: u128) -> Connection {
        Connection {
            id: ConnectionId::from_raw(id),
            source: PortEndpoint {
                component_id: ComponentId::from_raw(source),
                port_key: "out".into(),
            },
            target: PortEndpoint {
                component_id: ComponentId::from_raw(target),
                port_key: "in".into(),
            },
            label: None,
            route: vec![],
        }
    }

    fn model(components: Vec<ResolvedComponent>, connections: Vec<Connection>) -> ResolvedModel {
        ResolvedModel {
            document_id: DocumentId::from_raw(1),
            root: ResolvedSystem {
                id: SystemId::from_raw(10),
                document_id: DocumentId::from_raw(1),
                components,
                connections,
            },
            simulation: SimulationSettings {
                start_time: 0.0,
                stop_time: 1.0,
                timestep: 0.1,
                maximum_steps: 10,
                random_seed: 1,
                logging: LoggingPolicy::EveryStep,
                semantics: FixedStepSemantics::default(),
            },
            probes: vec![],
        }
    }

    #[test]
    fn fan_out_is_scheduled_once_with_stable_id_tie_breaks() {
        let schedule = build_schedule(&model(
            vec![component(3, true), component(1, false), component(2, true)],
            vec![connection(1, 1, 3), connection(2, 1, 2)],
        ))
        .unwrap();

        assert_eq!(
            schedule.component_order,
            vec![
                ComponentId::from_raw(1),
                ComponentId::from_raw(2),
                ComponentId::from_raw(3)
            ]
        );
    }

    #[test]
    fn state_boundary_permits_delayed_feedback() {
        let schedule = build_schedule(&model(
            vec![component(2, true), component(1, false)],
            vec![connection(1, 1, 2), connection(2, 2, 1)],
        ))
        .unwrap();

        assert_eq!(
            schedule.component_order,
            vec![ComponentId::from_raw(1), ComponentId::from_raw(2)]
        );
    }

    #[test]
    fn rejects_direct_loop_with_complete_component_path() {
        let failure = build_schedule(&model(
            vec![component(2, true), component(1, true)],
            vec![connection(1, 1, 2), connection(2, 2, 1)],
        ))
        .unwrap_err();

        assert_eq!(
            failure.component_path,
            vec![
                ComponentId::from_raw(1),
                ComponentId::from_raw(2),
                ComponentId::from_raw(1)
            ]
        );
        assert_eq!(
            failure.diagnostic.message_key().as_str(),
            "simulation_schedule_direct_feedthrough_loop"
        );
    }
}

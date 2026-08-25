//! Integer-exponent dimensional algebra over the seven SI base quantities.

use crate::{UnitFamilyId, UnitId};
use serde::{Deserialize, Serialize};

/// Exponents of length, mass, time, current, temperature, amount, and luminous intensity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Dimension {
    /// Length exponent.
    pub length: i8,
    /// Mass exponent.
    pub mass: i8,
    /// Time exponent.
    pub time: i8,
    /// Electric-current exponent.
    pub current: i8,
    /// Thermodynamic-temperature exponent.
    pub temperature: i8,
    /// Amount-of-substance exponent.
    pub amount: i8,
    /// Luminous-intensity exponent.
    pub luminous_intensity: i8,
}

impl Dimension {
    /// Dimensionless quantity.
    pub const DIMENSIONLESS: Self = Self::new(0, 0, 0, 0, 0, 0, 0);
    /// Length quantity, $L$.
    pub const LENGTH: Self = Self::new(1, 0, 0, 0, 0, 0, 0);
    /// Mass quantity, $M$.
    pub const MASS: Self = Self::new(0, 1, 0, 0, 0, 0, 0);
    /// Time quantity, $T$.
    pub const TIME: Self = Self::new(0, 0, 1, 0, 0, 0, 0);
    /// Electric-current quantity, $I$.
    pub const CURRENT: Self = Self::new(0, 0, 0, 1, 0, 0, 0);
    /// Thermodynamic-temperature quantity, $Theta$.
    pub const TEMPERATURE: Self = Self::new(0, 0, 0, 0, 1, 0, 0);
    /// Amount-of-substance quantity, $N$.
    pub const AMOUNT: Self = Self::new(0, 0, 0, 0, 0, 1, 0);
    /// Luminous-intensity quantity, $J$.
    pub const LUMINOUS_INTENSITY: Self = Self::new(0, 0, 0, 0, 0, 0, 1);
    /// Area quantity, $L^2$.
    pub const AREA: Self = Self::new(2, 0, 0, 0, 0, 0, 0);
    /// Volume quantity, $L^3$.
    pub const VOLUME: Self = Self::new(3, 0, 0, 0, 0, 0, 0);
    /// Velocity quantity, $L T^{-1}$.
    pub const VELOCITY: Self = Self::new(1, 0, -1, 0, 0, 0, 0);
    /// Acceleration quantity, $L T^{-2}$.
    pub const ACCELERATION: Self = Self::new(1, 0, -2, 0, 0, 0, 0);
    /// Force quantity, $M L T^{-2}$.
    pub const FORCE: Self = Self::new(1, 1, -2, 0, 0, 0, 0);
    /// Translational stiffness quantity, $M T^{-2}$.
    pub const STIFFNESS: Self = Self::new(0, 1, -2, 0, 0, 0, 0);
    /// Translational damping quantity, $M T^{-1}$.
    pub const DAMPING: Self = Self::new(0, 1, -1, 0, 0, 0, 0);
    /// Energy quantity, $M L^2 T^{-2}$.
    pub const ENERGY: Self = Self::new(2, 1, -2, 0, 0, 0, 0);
    /// Power quantity, $M L^2 T^{-3}$.
    pub const POWER: Self = Self::new(2, 1, -3, 0, 0, 0, 0);
    /// Pressure quantity, $M L^{-1} T^{-2}$.
    pub const PRESSURE: Self = Self::new(-1, 1, -2, 0, 0, 0, 0);
    /// Volumetric-flow quantity, $L^3 T^{-1}$.
    pub const VOLUME_FLOW: Self = Self::new(3, 0, -1, 0, 0, 0, 0);

    /// Creates a dimension from ordered SI base exponents.
    #[must_use]
    pub const fn new(
        length: i8,
        mass: i8,
        time: i8,
        current: i8,
        temperature: i8,
        amount: i8,
        luminous_intensity: i8,
    ) -> Self {
        Self {
            length,
            mass,
            time,
            current,
            temperature,
            amount,
            luminous_intensity,
        }
    }

    /// Multiplies quantities by adding their dimension exponents.
    ///
    /// # Errors
    ///
    /// Returns [`DimensionOverflow`] if any exponent exceeds the `i8` range.
    pub fn checked_mul(self, rhs: Self) -> Result<Self, DimensionOverflow> {
        self.checked_zip(rhs, i8::checked_add)
    }

    /// Divides quantities by subtracting their dimension exponents.
    ///
    /// # Errors
    ///
    /// Returns [`DimensionOverflow`] if any exponent exceeds the `i8` range.
    pub fn checked_div(self, rhs: Self) -> Result<Self, DimensionOverflow> {
        self.checked_zip(rhs, i8::checked_sub)
    }

    /// Raises a quantity to an integer power.
    ///
    /// # Errors
    ///
    /// Returns [`DimensionOverflow`] if any exponent exceeds the `i8` range.
    pub fn checked_pow(self, exponent: i8) -> Result<Self, DimensionOverflow> {
        let scale = |value: i8| value.checked_mul(exponent).ok_or(DimensionOverflow);
        Ok(Self::new(
            scale(self.length)?,
            scale(self.mass)?,
            scale(self.time)?,
            scale(self.current)?,
            scale(self.temperature)?,
            scale(self.amount)?,
            scale(self.luminous_intensity)?,
        ))
    }

    /// Applies one checked exponent operation component-wise.
    fn checked_zip(
        self,
        rhs: Self,
        operation: fn(i8, i8) -> Option<i8>,
    ) -> Result<Self, DimensionOverflow> {
        let apply = |left, right| operation(left, right).ok_or(DimensionOverflow);
        Ok(Self::new(
            apply(self.length, rhs.length)?,
            apply(self.mass, rhs.mass)?,
            apply(self.time, rhs.time)?,
            apply(self.current, rhs.current)?,
            apply(self.temperature, rhs.temperature)?,
            apply(self.amount, rhs.amount)?,
            apply(self.luminous_intensity, rhs.luminous_intensity)?,
        ))
    }
}

/// A dimension exponent could not be represented.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DimensionOverflow;

impl UnitFamilyId {
    /// Returns the physical dimension represented by this conversion family.
    #[must_use]
    pub const fn dimension(self) -> Dimension {
        match self {
            Self::None => Dimension::DIMENSIONLESS,
            Self::Area => Dimension::AREA,
            Self::Current => Dimension::CURRENT,
            Self::Length => Dimension::LENGTH,
            Self::LuminousIntensity => Dimension::LUMINOUS_INTENSITY,
            Self::Amount => Dimension::AMOUNT,
            Self::Temperature => Dimension::TEMPERATURE,
            Self::Time => Dimension::TIME,
            Self::Volume => Dimension::VOLUME,
            Self::Mass => Dimension::MASS,
        }
    }
}

impl UnitId {
    /// Returns the physical dimension represented by this concrete unit.
    #[must_use]
    pub const fn dimension(self) -> Dimension {
        self.family_id().dimension()
    }
}

#[cfg(test)]
mod tests {
    use super::{Dimension, DimensionOverflow};
    use crate::{UnitFamilyId, UnitId};

    #[test]
    fn derives_mechanical_quantities_from_base_dimensions() {
        assert_eq!(
            Dimension::LENGTH.checked_div(Dimension::TIME),
            Ok(Dimension::VELOCITY)
        );
        assert_eq!(
            Dimension::MASS.checked_mul(Dimension::ACCELERATION),
            Ok(Dimension::FORCE)
        );
        assert_eq!(
            Dimension::FORCE.checked_mul(Dimension::VELOCITY),
            Ok(Dimension::POWER)
        );
        assert_eq!(
            Dimension::FORCE.checked_div(Dimension::LENGTH),
            Ok(Dimension::STIFFNESS)
        );
    }

    #[test]
    fn rejects_exponent_overflow_without_wrapping() {
        let large = Dimension::new(i8::MAX, 0, 0, 0, 0, 0, 0);
        assert_eq!(large.checked_mul(Dimension::LENGTH), Err(DimensionOverflow));
        assert_eq!(Dimension::AREA.checked_pow(i8::MAX), Err(DimensionOverflow));
    }

    #[test]
    fn every_existing_unit_inherits_its_family_dimension() {
        for unit in UnitId::ALL {
            assert_eq!(unit.dimension(), unit.family_id().dimension());
        }
        assert_eq!(UnitFamilyId::Volume.dimension(), Dimension::VOLUME);
    }
}

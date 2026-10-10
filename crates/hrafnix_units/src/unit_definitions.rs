use hrafnix_keys::{ConstUnitKey, unit_key};

/// Defines [`UnitFamilyId`] and [`UnitId`] variants together with their metadata.
///
/// Families are listed in numeric and display order: the unitless sentinel first,
/// then supported SI base quantities, named SI derived quantities, and remaining
/// compound quantities. Units within a family list the base unit first, followed
/// by metric prefixed variants, followed by any other units.
macro_rules! define_units {
    ($(
        $(#[doc = $family_documentation:literal])*
        $family:ident = $family_value:literal => $family_description:literal {
            $(
                $unit:ident = $value:literal => (
                    $key:literal,
                    $description:literal,
                    $documentation:literal
                ),
            )*
        }
    )*) => {
        /// `UnitFamilyId` is an enum that represents the different families of units.
        /// Each family has a unique identifier that can be used to group units together.
        /// The `UnitFamilyId` enum is used in the Unit struct to specify the family of a unit.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[repr(u8)]
        pub enum UnitFamilyId {
            $(
                $(#[doc = $family_documentation])*
                $family = $family_value,
            )*
        }

        impl UnitFamilyId {
            const COUNT: usize = <[u8]>::len(&[$($family_value),*]);

            /// All supported unit families in numeric and display order.
            pub const ALL: [Self; Self::COUNT] = [
                $(
                    Self::$family,
                )*
            ];

            /// Returns the `UnitFamilyId` corresponding to the given u8 value.
            #[must_use]
            pub const fn from_u8(value: u8) -> Option<Self> {
                match value {
                    $(
                        $family_value => Some(Self::$family),
                    )*
                    _ => None,
                }
            }

            /// Returns the u8 value corresponding to the given `UnitFamilyId`.
            #[must_use]
            pub const fn to_u8(&self) -> u8 {
                match self {
                    $(
                        Self::$family => $family_value,
                    )*
                }
            }

            /// Returns the display name for this family.
            #[must_use]
            pub const fn description(&self) -> &'static str {
                match self {
                    $(
                        Self::$family => $family_description,
                    )*
                }
            }

            /// Returns the unit identifiers for all units in this family, in display order.
            #[must_use]
            pub const fn unit_ids(&self) -> &'static [UnitId] {
                match self {
                    $(
                        Self::$family => &[
                            $(
                                UnitId::$unit,
                            )*
                        ],
                    )*
                }
            }
        }

        /// Identifiers for the units supported by Hrafnix.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[repr(u16)]
        #[allow(non_camel_case_types)]
        pub enum UnitId {
            $($(
                #[doc = $documentation]
                $unit = $value,
            )*)*
        }

        impl UnitId {
            /// All supported unit identifiers.
            pub const ALL: &[Self] = &[
                $($(
                    Self::$unit,
                )*)*
            ];

            /// Returns the `UnitId` corresponding to the given string identifier.
            #[cfg_attr(feature = "hotpath", hotpath::measure)]
            pub fn from_unit_id_str(unit_id_str: &str) -> Option<Self> {
                match unit_id_str {
                    $($(
                        $key => Some(Self::$unit),
                    )*)*
                    _ => None,
                }
            }

            /// Returns the `UnitId` corresponding to the given u16 value.
            pub const fn from_u16(value: u16) -> Option<Self> {
                match value {
                    $($(
                        $value => Some(Self::$unit),
                    )*)*
                    _ => None,
                }
            }

            /// Returns the u16 value corresponding to the given `UnitId`.
            pub const fn to_u16(&self) -> u16 {
                match self {
                    $($(
                        Self::$unit => $value,
                    )*)*
                }
            }

            /// Returns the `UnitFamilyId` corresponding to the given `UnitId`.
            pub const fn family_id(&self) -> UnitFamilyId {
                match self {
                    $($(
                        Self::$unit => UnitFamilyId::$family,
                    )*)*
                }
            }

            /// Returns the string identifier corresponding to the given `UnitId`.
            pub const fn string_id(&self) -> ConstUnitKey {
                match self {
                    $($(
                        Self::$unit => unit_key!($key),
                    )*)*
                }
            }

            /// Returns the description corresponding to the given `UnitId`.
            pub const fn description(&self) -> &'static str {
                match self {
                    $($(
                        Self::$unit => $description,
                    )*)*
                }
            }

            /// Returns the documentation corresponding to the given `UnitId`.
            #[cfg(test)]
            const fn documentation(&self) -> &'static str {
                match self {
                    $($(
                        Self::$unit => $documentation,
                    )*)*
                }
            }
        }
    };
}

define_units! {
    /// The family of units that do not belong to any specific category.
    None = 0 => "None" {
        None = 0 => ("u_none", "", "No unit."),
    }

    /// The family of units that measure length.
    Length = 1 => "Length" {
        Length_Meter = 100 => ("u_length_meter", "m", "A meter."),
        Length_Picometer = 101 => ("u_length_picometer", "pm", "A picometer."),
        Length_Nanometer = 102 => ("u_length_nanometer", "nm", "A nanometer."),
        Length_Micrometer = 103 => ("u_length_micrometer", "μm", "A micrometer."),
        Length_Millimeter = 104 => ("u_length_millimeter", "mm", "A millimeter."),
        Length_Centimeter = 105 => ("u_length_centimeter", "cm", "A centimeter."),
        Length_Kilometer = 106 => ("u_length_kilometer", "km", "A kilometer."),
        Length_Foot = 107 => ("u_length_foot", "ft", "An international foot."),
        Length_Inch = 108 => ("u_length_inch", "in", "An international inch."),
        Length_Yard = 109 => ("u_length_yard", "yd", "An international yard."),
        Length_Mile = 110 => ("u_length_mile", "mi", "An international mile."),
    }

    /// The family of units that measure mass.
    Mass = 2 => "Mass" {
        Mass_Kilogram = 200 => ("u_mass_kilogram", "kg", "A kilogram."),
        Mass_Gram = 201 => ("u_mass_gram", "g", "A gram."),
        Mass_Picogram = 202 => ("u_mass_picogram", "pg", "A picogram."),
        Mass_Nanogram = 203 => ("u_mass_nanogram", "ng", "A nanogram."),
        Mass_Microgram = 204 => ("u_mass_microgram", "μg", "A microgram."),
        Mass_Milligram = 205 => ("u_mass_milligram", "mg", "A milligram."),
        Mass_Megagram = 206 => ("u_mass_megagram", "Mg", "A megagram."),
        Mass_Tonne = 207 => ("u_mass_tonne", "t", "A metric tonne."),
        Mass_Pound = 208 => ("u_mass_pound", "lb", "An international avoirdupois pound."),
        Mass_Ounce = 209 => ("u_mass_ounce", "oz", "An international avoirdupois ounce."),
        Mass_Stone = 210 => ("u_mass_stone", "st", "A stone."),
    }

    /// The family of units that measure time.
    Time = 3 => "Time" {
        Time_Second = 300 => ("u_time_second", "s", "A second."),
        Time_Picosecond = 301 => ("u_time_picosecond", "ps", "A picosecond."),
        Time_Nanosecond = 302 => ("u_time_nanosecond", "ns", "A nanosecond."),
        Time_Microsecond = 303 => ("u_time_microsecond", "μs", "A microsecond."),
        Time_Millisecond = 304 => ("u_time_millisecond", "ms", "A millisecond."),
        Time_Kilosecond = 305 => ("u_time_kilosecond", "ks", "A kilosecond."),
        Time_Minute = 306 => ("u_time_minute", "min", "A minute."),
        Time_Hour = 307 => ("u_time_hour", "h", "An hour."),
        Time_Day = 308 => ("u_time_day", "day", "A day."),
        Time_Week = 309 => ("u_time_week", "week", "A week."),
        Time_Year = 310 => ("u_time_year", "year", "A common year of 365 days."),
    }

    /// The family of units that measure electric current.
    Current = 4 => "Current" {
        Current_Ampere = 400 => ("u_current_ampere", "A", "An ampere."),
        Current_Nanoampere = 401 => ("u_current_nanoampere", "nA", "A nanoampere."),
        Current_Microampere = 402 => ("u_current_microampere", "μA", "A microampere."),
        Current_Milliampere = 403 => ("u_current_milliampere", "mA", "A milliampere."),
        Current_Kiloampere = 404 => ("u_current_kiloampere", "kA", "A kiloampere."),
        Current_Megaampere = 405 => ("u_current_megaampere", "MA", "A megaampere."),
    }

    /// The family of units that measure temperature.
    Temperature = 5 => "Temperature" {
        Temperature_Kelvin = 500 => ("u_temperature_kelvin", "K", "A kelvin."),
        Temperature_Nanokelvin = 501 => ("u_temperature_nanokelvin", "nK", "A nanokelvin."),
        Temperature_Microkelvin = 502 => ("u_temperature_microkelvin", "μK", "A microkelvin."),
        Temperature_Millikelvin = 503 => ("u_temperature_millikelvin", "mK", "A millikelvin."),
        Temperature_Kilokelvin = 504 => ("u_temperature_kilokelvin", "kK", "A kilokelvin."),
        Temperature_Celsius = 505 => ("u_temperature_celsius", "°C", "A degree Celsius, the SI unit for temperature relative to 273.15 K. A temperature interval of one degree Celsius equals one kelvin."),
        Temperature_Fahrenheit = 506 => ("u_temperature_fahrenheit", "°F", "A degree Fahrenheit."),
    }

    /// The family of units that measure the amount of substance.
    Amount = 6 => "Amount" {
        Amount_Mole = 600 => ("u_amount_mole", "mol", "A mole."),
        Amount_Picomole = 601 => ("u_amount_picomole", "pmol", "A picomole."),
        Amount_Nanomole = 602 => ("u_amount_nanomole", "nmol", "A nanomole."),
        Amount_Micromole = 603 => ("u_amount_micromole", "μmol", "A micromole."),
        Amount_Millimole = 604 => ("u_amount_millimole", "mmol", "A millimole."),
        Amount_Kilomole = 605 => ("u_amount_kilomole", "kmol", "A kilomole."),
    }

    /// The family of units that measure frequency.
    Frequency = 7 => "Frequency" {
        Frequency_Hertz = 700 => ("u_frequency_hertz", "Hz", "A hertz, the SI unit of frequency."),
        Frequency_Picohertz = 701 => ("u_frequency_picohertz", "pHz", "A picohertz."),
        Frequency_Nanohertz = 702 => ("u_frequency_nanohertz", "nHz", "A nanohertz."),
        Frequency_Microhertz = 703 => ("u_frequency_microhertz", "μHz", "A microhertz."),
        Frequency_Millihertz = 704 => ("u_frequency_millihertz", "mHz", "A millihertz."),
        Frequency_Kilohertz = 705 => ("u_frequency_kilohertz", "kHz", "A kilohertz."),
        Frequency_Megahertz = 706 => ("u_frequency_megahertz", "MHz", "A megahertz."),
        Frequency_Gigahertz = 707 => ("u_frequency_gigahertz", "GHz", "A gigahertz."),
        Frequency_Terahertz = 708 => ("u_frequency_terahertz", "THz", "A terahertz."),
    }

    /// The family of units that measure plane angle.
    Angle = 8 => "Angle" {
        Angle_Radian = 800 => ("u_angle_radian", "rad", "A radian, the SI unit of plane angle."),
        Angle_Picoradian = 801 => ("u_angle_picoradian", "prad", "A picoradian."),
        Angle_Nanoradian = 802 => ("u_angle_nanoradian", "nrad", "A nanoradian."),
        Angle_Microradian = 803 => ("u_angle_microradian", "μrad", "A microradian."),
        Angle_Milliradian = 804 => ("u_angle_milliradian", "mrad", "A milliradian."),
        Angle_Kiloradian = 805 => ("u_angle_kiloradian", "krad", "A kiloradian."),
        Angle_Megaradian = 806 => ("u_angle_megaradian", "Mrad", "A megaradian."),
        Angle_Gigaradian = 807 => ("u_angle_gigaradian", "Grad", "A gigaradian."),
        Angle_Teraradian = 808 => ("u_angle_teraradian", "Trad", "A teraradian."),
        Angle_Degree = 809 => ("u_angle_degree", "°", "A degree of plane angle."),
        Angle_Turn = 810 => ("u_angle_turn", "rev", "A turn or revolution."),
        Angle_Arcminute = 811 => ("u_angle_arcminute", "′", "An arcminute."),
        Angle_Arcsecond = 812 => ("u_angle_arcsecond", "″", "An arcsecond."),
    }

    /// The family of units that measure solid angle.
    SolidAngle = 9 => "Solid Angle" {
        SolidAngle_Steradian = 900 => ("u_solid_angle_steradian", "sr", "A steradian, the SI unit of solid angle."),
        SolidAngle_Picosteradian = 901 => ("u_solid_angle_picosteradian", "psr", "A picosteradian."),
        SolidAngle_Nanosteradian = 902 => ("u_solid_angle_nanosteradian", "nsr", "A nanosteradian."),
        SolidAngle_Microsteradian = 903 => ("u_solid_angle_microsteradian", "μsr", "A microsteradian."),
        SolidAngle_Millisteradian = 904 => ("u_solid_angle_millisteradian", "msr", "A millisteradian."),
    }

    /// The family of units that measure force or weight.
    Force = 10 => "Force" {
        Force_Newton = 1000 => ("u_force_newton", "N", "A newton, the SI unit of force or weight."),
        Force_Piconewton = 1001 => ("u_force_piconewton", "pN", "A piconewton."),
        Force_Nanonewton = 1002 => ("u_force_nanonewton", "nN", "A nanonewton."),
        Force_Micronewton = 1003 => ("u_force_micronewton", "μN", "A micronewton."),
        Force_Millinewton = 1004 => ("u_force_millinewton", "mN", "A millinewton."),
        Force_Kilonewton = 1005 => ("u_force_kilonewton", "kN", "A kilonewton."),
        Force_Meganewton = 1006 => ("u_force_meganewton", "MN", "A meganewton."),
        Force_Giganewton = 1007 => ("u_force_giganewton", "GN", "A giganewton."),
        Force_Teranewton = 1008 => ("u_force_teranewton", "TN", "A teranewton."),
        Force_KilogramForce = 1009 => ("u_force_kilogram_force", "kgf", "A kilogram-force."),
        Force_PoundForce = 1010 => ("u_force_pound_force", "lbf", "A pound-force."),
    }

    /// The family of units that measure pressure or stress.
    Pressure = 11 => "Pressure" {
        Pressure_Pascal = 1100 => ("u_pressure_pascal", "Pa", "A pascal, the SI unit of pressure or stress."),
        Pressure_Picopascal = 1101 => ("u_pressure_picopascal", "pPa", "A picopascal."),
        Pressure_Nanopascal = 1102 => ("u_pressure_nanopascal", "nPa", "A nanopascal."),
        Pressure_Micropascal = 1103 => ("u_pressure_micropascal", "μPa", "A micropascal."),
        Pressure_Millipascal = 1104 => ("u_pressure_millipascal", "mPa", "A millipascal."),
        Pressure_Kilopascal = 1105 => ("u_pressure_kilopascal", "kPa", "A kilopascal."),
        Pressure_Megapascal = 1106 => ("u_pressure_megapascal", "MPa", "A megapascal."),
        Pressure_Gigapascal = 1107 => ("u_pressure_gigapascal", "GPa", "A gigapascal."),
        Pressure_Terapascal = 1108 => ("u_pressure_terapascal", "TPa", "A terapascal."),
        Pressure_Bar = 1109 => ("u_pressure_bar", "bar", "A bar."),
        Pressure_Millibar = 1110 => ("u_pressure_millibar", "mbar", "A millibar."),
        Pressure_StandardAtmosphere = 1111 => ("u_pressure_standard_atmosphere", "atm", "A standard atmosphere."),
        Pressure_PoundPerSquareInch = 1112 => ("u_pressure_pound_per_square_inch", "psi", "A pound-force per square inch, using the international inch and standard gravity."),
        Pressure_MillimeterOfMercury = 1113 => ("u_pressure_millimeter_of_mercury", "mmHg", "A conventional millimeter of mercury."),
    }

    /// The family of units that measure energy, work, or heat.
    Energy = 12 => "Energy" {
        Energy_Joule = 1200 => ("u_energy_joule", "J", "A joule, the SI unit of energy, work, or heat."),
        Energy_Picojoule = 1201 => ("u_energy_picojoule", "pJ", "A picojoule."),
        Energy_Nanojoule = 1202 => ("u_energy_nanojoule", "nJ", "A nanojoule."),
        Energy_Microjoule = 1203 => ("u_energy_microjoule", "μJ", "A microjoule."),
        Energy_Millijoule = 1204 => ("u_energy_millijoule", "mJ", "A millijoule."),
        Energy_Kilojoule = 1205 => ("u_energy_kilojoule", "kJ", "A kilojoule."),
        Energy_Megajoule = 1206 => ("u_energy_megajoule", "MJ", "A megajoule."),
        Energy_Gigajoule = 1207 => ("u_energy_gigajoule", "GJ", "A gigajoule."),
        Energy_Terajoule = 1208 => ("u_energy_terajoule", "TJ", "A terajoule."),
        Energy_WattHour = 1209 => ("u_energy_watt_hour", "Wh", "A watt-hour."),
        Energy_KilowattHour = 1210 => ("u_energy_kilowatt_hour", "kWh", "A kilowatt-hour."),
        Energy_Electronvolt = 1211 => ("u_energy_electronvolt", "eV", "An electronvolt."),
    }

    /// The family of units that measure power or radiant flux.
    Power = 13 => "Power" {
        Power_Watt = 1300 => ("u_power_watt", "W", "A watt, the SI unit of power or radiant flux."),
        Power_Picowatt = 1301 => ("u_power_picowatt", "pW", "A picowatt."),
        Power_Nanowatt = 1302 => ("u_power_nanowatt", "nW", "A nanowatt."),
        Power_Microwatt = 1303 => ("u_power_microwatt", "μW", "A microwatt."),
        Power_Milliwatt = 1304 => ("u_power_milliwatt", "mW", "A milliwatt."),
        Power_Kilowatt = 1305 => ("u_power_kilowatt", "kW", "A kilowatt."),
        Power_Megawatt = 1306 => ("u_power_megawatt", "MW", "A megawatt."),
        Power_Gigawatt = 1307 => ("u_power_gigawatt", "GW", "A gigawatt."),
        Power_Terawatt = 1308 => ("u_power_terawatt", "TW", "A terawatt."),
        Power_MechanicalHorsepower = 1309 => ("u_power_mechanical_horsepower", "hp", "A mechanical horsepower."),
        Power_MetricHorsepower = 1310 => ("u_power_metric_horsepower", "PS", "A metric horsepower."),
    }

    /// The family of units that measure electric charge.
    ElectricCharge = 14 => "Electric Charge" {
        ElectricCharge_Coulomb = 1400 => ("u_electric_charge_coulomb", "C", "A coulomb, the SI unit of electric charge."),
        ElectricCharge_Picocoulomb = 1401 => ("u_electric_charge_picocoulomb", "pC", "A picocoulomb."),
        ElectricCharge_Nanocoulomb = 1402 => ("u_electric_charge_nanocoulomb", "nC", "A nanocoulomb."),
        ElectricCharge_Microcoulomb = 1403 => ("u_electric_charge_microcoulomb", "μC", "A microcoulomb."),
        ElectricCharge_Millicoulomb = 1404 => ("u_electric_charge_millicoulomb", "mC", "A millicoulomb."),
        ElectricCharge_Kilocoulomb = 1405 => ("u_electric_charge_kilocoulomb", "kC", "A kilocoulomb."),
        ElectricCharge_Megacoulomb = 1406 => ("u_electric_charge_megacoulomb", "MC", "A megacoulomb."),
        ElectricCharge_Gigacoulomb = 1407 => ("u_electric_charge_gigacoulomb", "GC", "A gigacoulomb."),
        ElectricCharge_Teracoulomb = 1408 => ("u_electric_charge_teracoulomb", "TC", "A teracoulomb."),
        ElectricCharge_AmpereHour = 1409 => ("u_electric_charge_ampere_hour", "Ah", "An ampere-hour."),
        ElectricCharge_MilliampereHour = 1410 => ("u_electric_charge_milliampere_hour", "mAh", "A milliampere-hour."),
    }

    /// The family of units that measure voltage or electric potential.
    Voltage = 15 => "Voltage" {
        Voltage_Volt = 1500 => ("u_voltage_volt", "V", "A volt, the SI unit of voltage, electric potential, or electromotive force."),
        Voltage_Picovolt = 1501 => ("u_voltage_picovolt", "pV", "A picovolt."),
        Voltage_Nanovolt = 1502 => ("u_voltage_nanovolt", "nV", "A nanovolt."),
        Voltage_Microvolt = 1503 => ("u_voltage_microvolt", "μV", "A microvolt."),
        Voltage_Millivolt = 1504 => ("u_voltage_millivolt", "mV", "A millivolt."),
        Voltage_Kilovolt = 1505 => ("u_voltage_kilovolt", "kV", "A kilovolt."),
        Voltage_Megavolt = 1506 => ("u_voltage_megavolt", "MV", "A megavolt."),
        Voltage_Gigavolt = 1507 => ("u_voltage_gigavolt", "GV", "A gigavolt."),
        Voltage_Teravolt = 1508 => ("u_voltage_teravolt", "TV", "A teravolt."),
    }

    /// The family of units that measure electrical resistance, reactance, or impedance.
    Resistance = 16 => "Resistance" {
        Resistance_Ohm = 1600 => ("u_resistance_ohm", "Ω", "An ohm, the SI unit of electrical resistance, reactance, or impedance."),
        Resistance_Picoohm = 1601 => ("u_resistance_picoohm", "pΩ", "A picoohm."),
        Resistance_Nanoohm = 1602 => ("u_resistance_nanoohm", "nΩ", "A nanoohm."),
        Resistance_Microohm = 1603 => ("u_resistance_microohm", "μΩ", "A microohm."),
        Resistance_Milliohm = 1604 => ("u_resistance_milliohm", "mΩ", "A milliohm."),
        Resistance_Kiloohm = 1605 => ("u_resistance_kiloohm", "kΩ", "A kiloohm."),
        Resistance_Megaohm = 1606 => ("u_resistance_megaohm", "MΩ", "A megaohm."),
        Resistance_Gigaohm = 1607 => ("u_resistance_gigaohm", "GΩ", "A gigaohm."),
        Resistance_Teraohm = 1608 => ("u_resistance_teraohm", "TΩ", "A teraohm."),
    }

    /// The family of units that measure electrical conductance, susceptance, or admittance.
    Conductance = 17 => "Conductance" {
        Conductance_Siemens = 1700 => ("u_conductance_siemens", "S", "A siemens, the SI unit of electrical conductance, susceptance, or admittance."),
        Conductance_Picosiemens = 1701 => ("u_conductance_picosiemens", "pS", "A picosiemens."),
        Conductance_Nanosiemens = 1702 => ("u_conductance_nanosiemens", "nS", "A nanosiemens."),
        Conductance_Microsiemens = 1703 => ("u_conductance_microsiemens", "μS", "A microsiemens."),
        Conductance_Millisiemens = 1704 => ("u_conductance_millisiemens", "mS", "A millisiemens."),
        Conductance_Kilosiemens = 1705 => ("u_conductance_kilosiemens", "kS", "A kilosiemens."),
        Conductance_Megasiemens = 1706 => ("u_conductance_megasiemens", "MS", "A megasiemens."),
        Conductance_Gigasiemens = 1707 => ("u_conductance_gigasiemens", "GS", "A gigasiemens."),
        Conductance_Terasiemens = 1708 => ("u_conductance_terasiemens", "TS", "A terasiemens."),
    }

    /// The family of units that measure capacitance.
    Capacitance = 18 => "Capacitance" {
        Capacitance_Farad = 1800 => ("u_capacitance_farad", "F", "A farad, the SI unit of capacitance."),
        Capacitance_Picofarad = 1801 => ("u_capacitance_picofarad", "pF", "A picofarad."),
        Capacitance_Nanofarad = 1802 => ("u_capacitance_nanofarad", "nF", "A nanofarad."),
        Capacitance_Microfarad = 1803 => ("u_capacitance_microfarad", "μF", "A microfarad."),
        Capacitance_Millifarad = 1804 => ("u_capacitance_millifarad", "mF", "A millifarad."),
        Capacitance_Kilofarad = 1805 => ("u_capacitance_kilofarad", "kF", "A kilofarad."),
    }

    /// The family of units that measure inductance or permeance.
    Inductance = 19 => "Inductance" {
        Inductance_Henry = 1900 => ("u_inductance_henry", "H", "A henry, the SI unit of inductance or permeance."),
        Inductance_Picohenry = 1901 => ("u_inductance_picohenry", "pH", "A picohenry."),
        Inductance_Nanohenry = 1902 => ("u_inductance_nanohenry", "nH", "A nanohenry."),
        Inductance_Microhenry = 1903 => ("u_inductance_microhenry", "μH", "A microhenry."),
        Inductance_Millihenry = 1904 => ("u_inductance_millihenry", "mH", "A millihenry."),
    }

    /// The family of units that measure magnetic flux density.
    MagneticFluxDensity = 20 => "Magnetic Flux Density" {
        MagneticFluxDensity_Tesla = 2000 => ("u_magnetic_flux_density_tesla", "T", "A tesla, the SI unit of magnetic flux density."),
        MagneticFluxDensity_Picotesla = 2001 => ("u_magnetic_flux_density_picotesla", "pT", "A picotesla."),
        MagneticFluxDensity_Nanotesla = 2002 => ("u_magnetic_flux_density_nanotesla", "nT", "A nanotesla."),
        MagneticFluxDensity_Microtesla = 2003 => ("u_magnetic_flux_density_microtesla", "μT", "A microtesla."),
        MagneticFluxDensity_Millitesla = 2004 => ("u_magnetic_flux_density_millitesla", "mT", "A millitesla."),
    }

    /// The family of units that measure magnetic flux.
    MagneticFlux = 21 => "Magnetic Flux" {
        MagneticFlux_Weber = 2100 => ("u_magnetic_flux_weber", "Wb", "A weber, the SI unit of magnetic flux."),
        MagneticFlux_Picoweber = 2101 => ("u_magnetic_flux_picoweber", "pWb", "A picoweber."),
        MagneticFlux_Nanoweber = 2102 => ("u_magnetic_flux_nanoweber", "nWb", "A nanoweber."),
        MagneticFlux_Microweber = 2103 => ("u_magnetic_flux_microweber", "μWb", "A microweber."),
        MagneticFlux_Milliweber = 2104 => ("u_magnetic_flux_milliweber", "mWb", "A milliweber."),
        MagneticFlux_Kiloweber = 2105 => ("u_magnetic_flux_kiloweber", "kWb", "A kiloweber."),
        MagneticFlux_Megaweber = 2106 => ("u_magnetic_flux_megaweber", "MWb", "A megaweber."),
        MagneticFlux_Gigaweber = 2107 => ("u_magnetic_flux_gigaweber", "GWb", "A gigaweber."),
        MagneticFlux_Teraweber = 2108 => ("u_magnetic_flux_teraweber", "TWb", "A teraweber."),
    }

    /// The family of units that measure area.
    Area = 22 => "Area" {
        Area_SquareMeter = 2200 => ("u_area_square_meter", "m²", "A square meter."),
        Area_SquareMicrometer = 2201 => ("u_area_square_micrometer", "μm²", "A square micrometer."),
        Area_SquareMillimeter = 2202 => ("u_area_square_millimeter", "mm²", "A square millimeter."),
        Area_SquareCentimeter = 2203 => ("u_area_square_centimeter", "cm²", "A square centimeter."),
        Area_SquareKilometer = 2204 => ("u_area_square_kilometer", "km²", "A square kilometer."),
        Area_SquareFoot = 2205 => ("u_area_square_foot", "ft²", "A square foot."),
        Area_SquareInch = 2206 => ("u_area_square_inch", "in²", "A square inch."),
        Area_Acre = 2207 => ("u_area_acre", "ac", "An international acre."),
        Area_Hectare = 2208 => ("u_area_hectare", "ha", "A hectare."),
        Area_SquareMile = 2209 => ("u_area_square_mile", "mi²", "A square international mile."),
    }

    /// The family of units that measure volume.
    Volume = 23 => "Volume" {
        Volume_CubicMeter = 2300 => ("u_volume_cubic_meter", "m³", "A cubic meter, the SI base for volume conversions."),
        Volume_CubicCentimeter = 2301 => ("u_volume_cubic_centimeter", "cm³", "A cubic centimeter."),
        Volume_CubicMillimeter = 2302 => ("u_volume_cubic_millimeter", "mm³", "A cubic millimeter."),
        Volume_Liter = 2303 => ("u_volume_liter", "L", "A liter."),
        Volume_Picoliter = 2304 => ("u_volume_picoliter", "pL", "A picoliter."),
        Volume_Nanoliter = 2305 => ("u_volume_nanoliter", "nL", "A nanoliter."),
        Volume_Microliter = 2306 => ("u_volume_microliter", "μL", "A microliter."),
        Volume_Milliliter = 2307 => ("u_volume_milliliter", "mL", "A milliliter."),
        Volume_Kiloliter = 2308 => ("u_volume_kiloliter", "kL", "A kiloliter."),
        Volume_Megaliter = 2309 => ("u_volume_megaliter", "ML", "A megaliter."),
        Volume_Gallon = 2310 => ("u_volume_gallon", "gal", "A US liquid gallon."),
        Volume_ImperialGallon = 2311 => ("u_volume_imperial_gallon", "imp gal", "An imperial gallon."),
        Volume_FluidOunce = 2312 => ("u_volume_fluid_ounce", "fl oz", "A US fluid ounce."),
        Volume_ImperialFluidOunce = 2313 => ("u_volume_imperial_fluid_ounce", "imp fl oz", "An imperial fluid ounce."),
        Volume_Cup = 2314 => ("u_volume_cup", "cup", "A US customary cup."),
        Volume_Pint = 2315 => ("u_volume_pint", "pt", "A US liquid pint."),
        Volume_Quart = 2316 => ("u_volume_quart", "qt", "A US liquid quart."),
    }

    /// The family of units that measure speed.
    Speed = 24 => "Speed" {
        Speed_MeterPerSecond = 2400 => ("u_speed_meter_per_second", "m/s", "A meter per second, the SI unit of speed."),
        Speed_MillimeterPerSecond = 2401 => ("u_speed_millimeter_per_second", "mm/s", "A millimeter per second."),
        Speed_CentimeterPerSecond = 2402 => ("u_speed_centimeter_per_second", "cm/s", "A centimeter per second."),
        Speed_KilometerPerHour = 2403 => ("u_speed_kilometer_per_hour", "km/h", "A kilometer per hour."),
        Speed_InchPerSecond = 2404 => ("u_speed_inch_per_second", "in/s", "An international inch per second."),
        Speed_FootPerSecond = 2405 => ("u_speed_foot_per_second", "ft/s", "An international foot per second."),
        Speed_MilePerHour = 2406 => ("u_speed_mile_per_hour", "mph", "An international mile per hour."),
        Speed_Knot = 2407 => ("u_speed_knot", "kn", "A knot."),
    }

    /// The family of units that measure acceleration.
    Acceleration = 25 => "Acceleration" {
        Acceleration_MeterPerSecondSquared = 2500 => ("u_acceleration_meter_per_second_squared", "m/s²", "A meter per second squared, the SI unit of acceleration."),
        Acceleration_MillimeterPerSecondSquared = 2501 => ("u_acceleration_millimeter_per_second_squared", "mm/s²", "A millimeter per second squared."),
        Acceleration_CentimeterPerSecondSquared = 2502 => ("u_acceleration_centimeter_per_second_squared", "cm/s²", "A centimeter per second squared."),
        Acceleration_InchPerSecondSquared = 2503 => ("u_acceleration_inch_per_second_squared", "in/s²", "An international inch per second squared."),
        Acceleration_FootPerSecondSquared = 2504 => ("u_acceleration_foot_per_second_squared", "ft/s²", "An international foot per second squared."),
        Acceleration_StandardGravity = 2505 => ("u_acceleration_standard_gravity", "g₀", "Standard gravity."),
    }

    /// The family of units that measure angular velocity.
    AngularVelocity = 26 => "Angular Velocity" {
        AngularVelocity_RadianPerSecond = 2600 => ("u_angular_velocity_radian_per_second", "rad/s", "A radian per second, the SI unit of angular velocity."),
        AngularVelocity_DegreePerSecond = 2601 => ("u_angular_velocity_degree_per_second", "°/s", "A degree per second."),
        AngularVelocity_RevolutionPerMinute = 2602 => ("u_angular_velocity_revolution_per_minute", "rpm", "A revolution per minute."),
        AngularVelocity_RevolutionPerSecond = 2603 => ("u_angular_velocity_revolution_per_second", "rev/s", "A revolution per second."),
    }

    /// The family of units that measure torque.
    Torque = 27 => "Torque" {
        Torque_NewtonMeter = 2700 => ("u_torque_newton_meter", "N·m", "A newton meter, the SI unit of torque, distinct from energy despite sharing its SI dimensions."),
        Torque_NewtonMillimeter = 2701 => ("u_torque_newton_millimeter", "N·mm", "A newton millimeter."),
        Torque_NewtonCentimeter = 2702 => ("u_torque_newton_centimeter", "N·cm", "A newton centimeter."),
        Torque_PoundForceInch = 2703 => ("u_torque_pound_force_inch", "lbf·in", "A pound-force inch."),
        Torque_PoundForceFoot = 2704 => ("u_torque_pound_force_foot", "lbf·ft", "A pound-force foot."),
    }

    /// The family of units that measure mass density.
    Density = 28 => "Density" {
        Density_KilogramPerCubicMeter = 2800 => ("u_density_kilogram_per_cubic_meter", "kg/m³", "A kilogram per cubic meter, the SI unit of mass density."),
        Density_GramPerCubicCentimeter = 2801 => ("u_density_gram_per_cubic_centimeter", "g/cm³", "A gram per cubic centimeter."),
        Density_GramPerLiter = 2802 => ("u_density_gram_per_liter", "g/L", "A gram per liter."),
        Density_KilogramPerLiter = 2803 => ("u_density_kilogram_per_liter", "kg/L", "A kilogram per liter."),
        Density_PoundPerCubicFoot = 2804 => ("u_density_pound_per_cubic_foot", "lb/ft³", "An international avoirdupois pound per cubic international foot."),
    }

    /// The family of units that measure volume flow rate.
    VolumeFlowRate = 29 => "Volume Flow Rate" {
        VolumeFlowRate_CubicMeterPerSecond = 2900 => ("u_volume_flow_rate_cubic_meter_per_second", "m³/s", "A cubic meter per second, the SI unit of volume flow rate."),
        VolumeFlowRate_LiterPerMinute = 2901 => ("u_volume_flow_rate_liter_per_minute", "L/min", "A liter per minute."),
    }

    /// The family of units that measure pump or motor displacement per revolution.
    DisplacementPerRevolution = 30 => "Displacement per Revolution" {
        DisplacementPerRevolution_CubicMeterPerRevolution = 3000 => ("u_displacement_per_revolution_cubic_meter_per_revolution", "m³/rev", "A cubic meter per revolution, measuring pump or motor displacement."),
        DisplacementPerRevolution_CubicCentimeterPerRevolution = 3001 => ("u_displacement_per_revolution_cubic_centimeter_per_revolution", "cm³/rev", "A cubic centimeter per revolution."),
    }

    /// The family of units that measure angular frequency, distinct from frequency and angular velocity.
    AngularFrequency = 31 => "Angular Frequency" {
        AngularFrequency_RadianPerSecond = 3100 => ("u_angular_frequency_radian_per_second", "rad/s", "A radian per second, the SI unit of angular frequency, related to cyclic frequency by omega = 2*pi*f."),
        AngularFrequency_PicoradianPerSecond = 3101 => ("u_angular_frequency_picoradian_per_second", "prad/s", "A picoradian per second."),
        AngularFrequency_NanoradianPerSecond = 3102 => ("u_angular_frequency_nanoradian_per_second", "nrad/s", "A nanoradian per second."),
        AngularFrequency_MicroradianPerSecond = 3103 => ("u_angular_frequency_microradian_per_second", "μrad/s", "A microradian per second."),
        AngularFrequency_MilliradianPerSecond = 3104 => ("u_angular_frequency_milliradian_per_second", "mrad/s", "A milliradian per second."),
        AngularFrequency_KiloradianPerSecond = 3105 => ("u_angular_frequency_kiloradian_per_second", "krad/s", "A kiloradian per second."),
        AngularFrequency_MegaradianPerSecond = 3106 => ("u_angular_frequency_megaradian_per_second", "Mrad/s", "A megaradian per second."),
        AngularFrequency_GigaradianPerSecond = 3107 => ("u_angular_frequency_gigaradian_per_second", "Grad/s", "A gigaradian per second."),
        AngularFrequency_TeraradianPerSecond = 3108 => ("u_angular_frequency_teraradian_per_second", "Trad/s", "A teraradian per second."),
    }

    /// The family of units that measure rotational inertia.
    MomentOfInertia = 32 => "Moment of Inertia" {
        MomentOfInertia_KilogramSquareMeter = 3200 => ("u_moment_of_inertia_kilogram_square_meter", "kg·m²", "A kilogram square meter, the SI unit of moment of inertia."),
        MomentOfInertia_KilogramSquareCentimeter = 3201 => ("u_moment_of_inertia_kilogram_square_centimeter", "kg·cm²", "A kilogram square centimeter."),
        MomentOfInertia_GramSquareCentimeter = 3202 => ("u_moment_of_inertia_gram_square_centimeter", "g·cm²", "A gram square centimeter."),
    }

    /// The family of units that measure force per linear displacement.
    LinearStiffness = 33 => "Linear Stiffness" {
        LinearStiffness_NewtonPerMeter = 3300 => ("u_linear_stiffness_newton_per_meter", "N/m", "A newton per meter, the SI unit of linear stiffness."),
        LinearStiffness_NewtonPerMillimeter = 3301 => ("u_linear_stiffness_newton_per_millimeter", "N/mm", "A newton per millimeter."),
        LinearStiffness_KilonewtonPerMeter = 3302 => ("u_linear_stiffness_kilonewton_per_meter", "kN/m", "A kilonewton per meter."),
    }

    /// The family of units that measure force per linear velocity.
    LinearDamping = 34 => "Linear Damping" {
        LinearDamping_NewtonSecondPerMeter = 3400 => ("u_linear_damping_newton_second_per_meter", "N·s/m", "A newton second per meter, the SI unit of linear damping."),
        LinearDamping_NewtonSecondPerMillimeter = 3401 => ("u_linear_damping_newton_second_per_millimeter", "N·s/mm", "A newton second per millimeter."),
        LinearDamping_KilonewtonSecondPerMeter = 3402 => ("u_linear_damping_kilonewton_second_per_meter", "kN·s/m", "A kilonewton second per meter."),
    }

    /// The family of units that measure torque per angular displacement.
    RotationalStiffness = 35 => "Rotational Stiffness" {
        RotationalStiffness_NewtonMeterPerRadian = 3500 => ("u_rotational_stiffness_newton_meter_per_radian", "N·m/rad", "A newton meter per radian, the SI unit of rotational stiffness, distinct from torque."),
        RotationalStiffness_NewtonMillimeterPerRadian = 3501 => ("u_rotational_stiffness_newton_millimeter_per_radian", "N·mm/rad", "A newton millimeter per radian."),
        RotationalStiffness_KilonewtonMeterPerRadian = 3502 => ("u_rotational_stiffness_kilonewton_meter_per_radian", "kN·m/rad", "A kilonewton meter per radian."),
    }

    /// The family of units that measure torque per angular velocity.
    RotationalDamping = 36 => "Rotational Damping" {
        RotationalDamping_NewtonMeterSecondPerRadian = 3600 => ("u_rotational_damping_newton_meter_second_per_radian", "N·m·s/rad", "A newton meter second per radian, the SI unit of rotational damping."),
        RotationalDamping_NewtonMillimeterSecondPerRadian = 3601 => ("u_rotational_damping_newton_millimeter_second_per_radian", "N·mm·s/rad", "A newton millimeter second per radian."),
        RotationalDamping_KilonewtonMeterSecondPerRadian = 3602 => ("u_rotational_damping_kilonewton_meter_second_per_radian", "kN·m·s/rad", "A kilonewton meter second per radian."),
    }

    /// The family of units that measure dynamic viscosity.
    DynamicViscosity = 37 => "Dynamic Viscosity" {
        DynamicViscosity_PascalSecond = 3700 => ("u_dynamic_viscosity_pascal_second", "Pa·s", "A pascal second, the SI unit of dynamic viscosity."),
        DynamicViscosity_MillipascalSecond = 3701 => ("u_dynamic_viscosity_millipascal_second", "mPa·s", "A millipascal second."),
        DynamicViscosity_Poise = 3702 => ("u_dynamic_viscosity_poise", "P", "A poise."),
        DynamicViscosity_Centipoise = 3703 => ("u_dynamic_viscosity_centipoise", "cP", "A centipoise."),
    }

    /// The family of units that measure kinematic viscosity.
    KinematicViscosity = 38 => "Kinematic Viscosity" {
        KinematicViscosity_SquareMeterPerSecond = 3800 => ("u_kinematic_viscosity_square_meter_per_second", "m²/s", "A square meter per second, the SI unit of kinematic viscosity."),
        KinematicViscosity_SquareMillimeterPerSecond = 3801 => ("u_kinematic_viscosity_square_millimeter_per_second", "mm²/s", "A square millimeter per second."),
        KinematicViscosity_Stokes = 3802 => ("u_kinematic_viscosity_stokes", "St", "A stokes."),
        KinematicViscosity_Centistokes = 3803 => ("u_kinematic_viscosity_centistokes", "cSt", "A centistokes."),
    }

    /// The family of units that measure mass flow rate.
    MassFlowRate = 39 => "Mass Flow Rate" {
        MassFlowRate_KilogramPerSecond = 3900 => ("u_mass_flow_rate_kilogram_per_second", "kg/s", "A kilogram per second, the SI unit of mass flow rate."),
        MassFlowRate_GramPerSecond = 3901 => ("u_mass_flow_rate_gram_per_second", "g/s", "A gram per second."),
        MassFlowRate_KilogramPerMinute = 3902 => ("u_mass_flow_rate_kilogram_per_minute", "kg/min", "A kilogram per minute."),
        MassFlowRate_KilogramPerHour = 3903 => ("u_mass_flow_rate_kilogram_per_hour", "kg/h", "A kilogram per hour."),
    }

    /// The family of units that measure linear momentum.
    Momentum = 40 => "Momentum" {
        Momentum_KilogramMeterPerSecond = 4000 => ("u_momentum_kilogram_meter_per_second", "kg·m/s", "A kilogram meter per second, the SI unit of linear momentum."),
        Momentum_GramCentimeterPerSecond = 4001 => ("u_momentum_gram_centimeter_per_second", "g·cm/s", "A gram centimeter per second."),
        Momentum_NewtonSecond = 4002 => ("u_momentum_newton_second", "N·s", "A newton second."),
    }

    /// The family of units that measure hydraulic leakage flow per pressure difference.
    HydraulicLeakageCoefficient = 41 => "Hydraulic Leakage Coefficient" {
        HydraulicLeakageCoefficient_CubicMeterPerSecondPerPascal = 4100 => ("u_hydraulic_leakage_coefficient_cubic_meter_per_second_per_pascal", "m³/s/Pa", "A cubic meter per second per pascal, the SI unit of hydraulic leakage flow per pressure difference."),
        HydraulicLeakageCoefficient_LiterPerMinutePerBar = 4101 => ("u_hydraulic_leakage_coefficient_liter_per_minute_per_bar", "L/min/bar", "A liter per minute per bar."),
    }

    /// The family of units that measure heat capacity per unit mass.
    SpecificHeatCapacity = 42 => "Specific Heat Capacity" {
        SpecificHeatCapacity_JoulePerKilogramKelvin = 4200 => ("u_specific_heat_capacity_joule_per_kilogram_kelvin", "J/kg/K", "A joule per kilogram per kelvin, the SI unit of specific heat capacity, distinct from the specific gas constant."),
        SpecificHeatCapacity_KilojoulePerKilogramKelvin = 4201 => ("u_specific_heat_capacity_kilojoule_per_kilogram_kelvin", "kJ/kg/K", "A kilojoule per kilogram per kelvin."),
    }

    /// The family of units that measure the gas constant per unit mass.
    SpecificGasConstant = 43 => "Specific Gas Constant" {
        SpecificGasConstant_JoulePerKilogramKelvin = 4300 => ("u_specific_gas_constant_joule_per_kilogram_kelvin", "J/kg/K", "A joule per kilogram per kelvin, the SI unit of the specific gas constant, distinct from specific heat capacity."),
        SpecificGasConstant_KilojoulePerKilogramKelvin = 4301 => ("u_specific_gas_constant_kilojoule_per_kilogram_kelvin", "kJ/kg/K", "A kilojoule per kilogram per kelvin for the specific gas constant."),
    }

    /// The family of units that measure heat flow per temperature difference.
    ThermalConductance = 44 => "Thermal Conductance" {
        ThermalConductance_WattPerKelvin = 4400 => ("u_thermal_conductance_watt_per_kelvin", "W/K", "A watt per kelvin, the SI unit of heat flow per temperature difference."),
        ThermalConductance_MilliwattPerKelvin = 4401 => ("u_thermal_conductance_milliwatt_per_kelvin", "mW/K", "A milliwatt per kelvin."),
        ThermalConductance_KilowattPerKelvin = 4402 => ("u_thermal_conductance_kilowatt_per_kelvin", "kW/K", "A kilowatt per kelvin."),
    }

    /// The family of units that measure pressure difference per volume flow.
    HydraulicResistance = 45 => "Hydraulic Resistance" {
        HydraulicResistance_PascalSecondPerCubicMeter = 4500 => ("u_hydraulic_resistance_pascal_second_per_cubic_meter", "Pa·s/m³", "A pascal second per cubic meter, the SI unit of hydraulic resistance or hydraulic characteristic impedance, reciprocal in dimension to a hydraulic leakage coefficient."),
        HydraulicResistance_BarMinutePerLiter = 4501 => ("u_hydraulic_resistance_bar_minute_per_liter", "bar·min/L", "A bar minute per liter."),
    }

    /// The family of units that measure pneumatic pressure per energy flow.
    PneumaticCharacteristicImpedance = 46 => "Pneumatic Characteristic Impedance" {
        PneumaticCharacteristicImpedance_PascalSecondPerJoule = 4600 => ("u_pneumatic_characteristic_impedance_pascal_second_per_joule", "Pa·s/J", "A pascal second per joule, the SI unit of pneumatic pressure per energy flow, distinct from hydraulic impedance."),
        PneumaticCharacteristicImpedance_SecondPerCubicMeter = 4601 => ("u_pneumatic_characteristic_impedance_second_per_cubic_meter", "s/m³", "A second per cubic meter."),
    }

    /// The family of units that measure volume flow per square root of pressure difference.
    TurbulentFlowCoefficient = 47 => "Turbulent Flow Coefficient" {
        TurbulentFlowCoefficient_CubicMeterPerSecondPerSquareRootPascal = 4700 => ("u_turbulent_flow_coefficient_cubic_meter_per_second_per_square_root_pascal", "m³/s/√Pa", "A cubic meter per second per square root of a pascal, measuring turbulent flow per square root of pressure difference, distinct from linear leakage."),
        TurbulentFlowCoefficient_LiterPerMinutePerSquareRootBar = 4701 => ("u_turbulent_flow_coefficient_liter_per_minute_per_square_root_bar", "L/min/√bar", "A liter per minute per square root of a bar."),
    }

    /// The family of units that measure back electromotive force per angular velocity.
    MotorBackEmfConstant = 48 => "Motor Back EMF Constant" {
        MotorBackEmfConstant_VoltSecondPerRadian = 4800 => ("u_motor_back_emf_constant_volt_second_per_radian", "V·s/rad", "A volt second per radian, measuring motor back electromotive force per angular velocity."),
        MotorBackEmfConstant_MillivoltSecondPerRadian = 4801 => ("u_motor_back_emf_constant_millivolt_second_per_radian", "mV·s/rad", "A millivolt second per radian."),
        MotorBackEmfConstant_VoltPerRevolutionPerMinute = 4802 => ("u_motor_back_emf_constant_volt_per_revolution_per_minute", "V/rpm", "A volt per revolution per minute."),
    }

    /// The family of units that measure linear travel per angular displacement.
    ScrewPitch = 49 => "Screw Pitch" {
        ScrewPitch_MeterPerRadian = 4900 => ("u_screw_pitch_meter_per_radian", "m/rad", "A meter per radian, measuring linear travel per angular displacement."),
        ScrewPitch_MillimeterPerRadian = 4901 => ("u_screw_pitch_millimeter_per_radian", "mm/rad", "A millimeter per radian."),
        ScrewPitch_MillimeterPerRevolution = 4902 => ("u_screw_pitch_millimeter_per_revolution", "mm/rev", "A millimeter per revolution."),
    }

    /// The family of units that measure fuel mass flow per thrust.
    ThrustSpecificFuelConsumption = 50 => "Thrust-Specific Fuel Consumption" {
        ThrustSpecificFuelConsumption_KilogramPerNewtonSecond = 5000 => ("u_thrust_specific_fuel_consumption_kilogram_per_newton_second", "kg/N/s", "A kilogram per newton per second, measuring fuel mass flow per thrust."),
        ThrustSpecificFuelConsumption_KilogramPerKilonewtonHour = 5001 => ("u_thrust_specific_fuel_consumption_kilogram_per_kilonewton_hour", "kg/kN/h", "A kilogram per kilonewton per hour."),
    }

    /// The family of units that measure the time derivative of volume flow.
    VolumeFlowAcceleration = 51 => "Volume Flow Acceleration" {
        VolumeFlowAcceleration_CubicMeterPerSecondSquared = 5100 => ("u_volume_flow_acceleration_cubic_meter_per_second_squared", "m³/s²", "A cubic meter per second squared, the SI unit of the time derivative of volume flow."),
        VolumeFlowAcceleration_LiterPerMinutePerSecond = 5101 => ("u_volume_flow_acceleration_liter_per_minute_per_second", "L/min/s", "A liter per minute per second."),
    }
}

#[cfg(test)]
mod tests {
    use super::{UnitFamilyId, UnitId};
    use std::collections::HashSet;

    #[test]
    fn families_follow_si_category_order() {
        let expected = [
            UnitFamilyId::None,
            UnitFamilyId::Length,
            UnitFamilyId::Mass,
            UnitFamilyId::Time,
            UnitFamilyId::Current,
            UnitFamilyId::Temperature,
            UnitFamilyId::Amount,
            UnitFamilyId::Frequency,
            UnitFamilyId::Angle,
            UnitFamilyId::SolidAngle,
            UnitFamilyId::Force,
            UnitFamilyId::Pressure,
            UnitFamilyId::Energy,
            UnitFamilyId::Power,
            UnitFamilyId::ElectricCharge,
            UnitFamilyId::Voltage,
            UnitFamilyId::Resistance,
            UnitFamilyId::Conductance,
            UnitFamilyId::Capacitance,
            UnitFamilyId::Inductance,
            UnitFamilyId::MagneticFluxDensity,
            UnitFamilyId::MagneticFlux,
            UnitFamilyId::Area,
            UnitFamilyId::Volume,
            UnitFamilyId::Speed,
            UnitFamilyId::Acceleration,
            UnitFamilyId::AngularVelocity,
            UnitFamilyId::Torque,
            UnitFamilyId::Density,
            UnitFamilyId::VolumeFlowRate,
            UnitFamilyId::DisplacementPerRevolution,
            UnitFamilyId::AngularFrequency,
            UnitFamilyId::MomentOfInertia,
            UnitFamilyId::LinearStiffness,
            UnitFamilyId::LinearDamping,
            UnitFamilyId::RotationalStiffness,
            UnitFamilyId::RotationalDamping,
            UnitFamilyId::DynamicViscosity,
            UnitFamilyId::KinematicViscosity,
            UnitFamilyId::MassFlowRate,
            UnitFamilyId::Momentum,
            UnitFamilyId::HydraulicLeakageCoefficient,
            UnitFamilyId::SpecificHeatCapacity,
            UnitFamilyId::SpecificGasConstant,
            UnitFamilyId::ThermalConductance,
            UnitFamilyId::HydraulicResistance,
            UnitFamilyId::PneumaticCharacteristicImpedance,
            UnitFamilyId::TurbulentFlowCoefficient,
            UnitFamilyId::MotorBackEmfConstant,
            UnitFamilyId::ScrewPitch,
            UnitFamilyId::ThrustSpecificFuelConsumption,
            UnitFamilyId::VolumeFlowAcceleration,
        ];
        assert_eq!(UnitFamilyId::ALL, expected);
    }

    #[test]
    fn si_base_units_are_registered_first_in_their_families() {
        let cases = [
            (UnitId::Length_Meter, 100),
            (UnitId::Mass_Kilogram, 200),
            (UnitId::Time_Second, 300),
            (UnitId::Current_Ampere, 400),
            (UnitId::Temperature_Kelvin, 500),
            (UnitId::Amount_Mole, 600),
        ];
        for (unit, id) in cases {
            assert_eq!(unit.to_u16(), id);
            assert_eq!(UnitId::from_u16(id), Some(unit));
            assert_eq!(unit.family_id().unit_ids().first(), Some(&unit));
        }
    }

    #[test]
    fn hydraulic_units_are_registered() {
        let cases = [
            (
                UnitId::VolumeFlowRate_CubicMeterPerSecond,
                "u_volume_flow_rate_cubic_meter_per_second",
                2900,
                "m³/s",
            ),
            (
                UnitId::VolumeFlowRate_LiterPerMinute,
                "u_volume_flow_rate_liter_per_minute",
                2901,
                "L/min",
            ),
            (
                UnitId::DisplacementPerRevolution_CubicMeterPerRevolution,
                "u_displacement_per_revolution_cubic_meter_per_revolution",
                3000,
                "m³/rev",
            ),
            (
                UnitId::DisplacementPerRevolution_CubicCentimeterPerRevolution,
                "u_displacement_per_revolution_cubic_centimeter_per_revolution",
                3001,
                "cm³/rev",
            ),
        ];

        for (unit, key, id, symbol) in cases {
            assert_eq!(UnitId::from_unit_id_str(key), Some(unit));
            assert_eq!(UnitId::from_u16(id), Some(unit));
            assert_eq!(unit.description(), symbol);
        }
    }

    #[test]
    fn unit_family_ids_are_consistent() {
        let mut family_values = HashSet::new();
        let mut family_keys = HashSet::new();

        for family in UnitFamilyId::ALL {
            let value = family.to_u8();
            let key = family.description().to_string();

            assert!(
                family_values.insert(value),
                "duplicate family value: {value}"
            );
            assert!(
                family_keys.insert(key.clone()),
                "duplicate family key: {key}"
            );
            assert_eq!(UnitFamilyId::from_u8(value), Some(family));
            assert_ne!(family.description(), "");
            assert!(!family.description().contains(['(', ')']));

            let description = match family {
                UnitFamilyId::None => "None",
                UnitFamilyId::Length => "Length",
                UnitFamilyId::Mass => "Mass",
                UnitFamilyId::Time => "Time",
                UnitFamilyId::Current => "Current",
                UnitFamilyId::Temperature => "Temperature",
                UnitFamilyId::Amount => "Amount",
                UnitFamilyId::Frequency => "Frequency",
                UnitFamilyId::Angle => "Angle",
                UnitFamilyId::SolidAngle => "Solid Angle",
                UnitFamilyId::Force => "Force",
                UnitFamilyId::Pressure => "Pressure",
                UnitFamilyId::Energy => "Energy",
                UnitFamilyId::Power => "Power",
                UnitFamilyId::ElectricCharge => "Electric Charge",
                UnitFamilyId::Voltage => "Voltage",
                UnitFamilyId::Resistance => "Resistance",
                UnitFamilyId::Conductance => "Conductance",
                UnitFamilyId::Capacitance => "Capacitance",
                UnitFamilyId::Inductance => "Inductance",
                UnitFamilyId::MagneticFluxDensity => "Magnetic Flux Density",
                UnitFamilyId::MagneticFlux => "Magnetic Flux",
                UnitFamilyId::Area => "Area",
                UnitFamilyId::Volume => "Volume",
                UnitFamilyId::Speed => "Speed",
                UnitFamilyId::Acceleration => "Acceleration",
                UnitFamilyId::AngularVelocity => "Angular Velocity",
                UnitFamilyId::Torque => "Torque",
                UnitFamilyId::Density => "Density",
                UnitFamilyId::VolumeFlowRate => "Volume Flow Rate",
                UnitFamilyId::DisplacementPerRevolution => "Displacement per Revolution",
                UnitFamilyId::AngularFrequency => "Angular Frequency",
                UnitFamilyId::MomentOfInertia => "Moment of Inertia",
                UnitFamilyId::LinearStiffness => "Linear Stiffness",
                UnitFamilyId::LinearDamping => "Linear Damping",
                UnitFamilyId::RotationalStiffness => "Rotational Stiffness",
                UnitFamilyId::RotationalDamping => "Rotational Damping",
                UnitFamilyId::DynamicViscosity => "Dynamic Viscosity",
                UnitFamilyId::KinematicViscosity => "Kinematic Viscosity",
                UnitFamilyId::MassFlowRate => "Mass Flow Rate",
                UnitFamilyId::Momentum => "Momentum",
                UnitFamilyId::HydraulicLeakageCoefficient => "Hydraulic Leakage Coefficient",
                UnitFamilyId::SpecificHeatCapacity => "Specific Heat Capacity",
                UnitFamilyId::SpecificGasConstant => "Specific Gas Constant",
                UnitFamilyId::ThermalConductance => "Thermal Conductance",
                UnitFamilyId::HydraulicResistance => "Hydraulic Resistance",
                UnitFamilyId::PneumaticCharacteristicImpedance => {
                    "Pneumatic Characteristic Impedance"
                }
                UnitFamilyId::TurbulentFlowCoefficient => "Turbulent Flow Coefficient",
                UnitFamilyId::MotorBackEmfConstant => "Motor Back EMF Constant",
                UnitFamilyId::ScrewPitch => "Screw Pitch",
                UnitFamilyId::ThrustSpecificFuelConsumption => "Thrust-Specific Fuel Consumption",
                UnitFamilyId::VolumeFlowAcceleration => "Volume Flow Acceleration",
            };
            assert_eq!(
                family.description(),
                description,
                "family description does not match expected value: {}",
                family.description()
            );
        }
    }

    #[test]
    fn unit_symbols_are_unique_within_each_family() {
        for family in UnitFamilyId::ALL {
            let mut symbols = HashSet::new();
            for unit in family.unit_ids() {
                let symbol = unit.description();
                assert!(
                    symbols.insert(symbol),
                    "duplicate symbol {symbol:?} in {family:?}"
                );
            }
        }
    }

    #[test]
    fn base_families_have_expected_engineering_prefixes() {
        let prefixes = [
            "pico", "nano", "micro", "milli", "kilo", "mega", "giga", "tera",
        ];
        // Base units are deliberately limited to prefixes with engineering relevance.
        let cases: &[(UnitFamilyId, &str, &[&str])] = &[
            (
                UnitFamilyId::Length,
                "meter",
                &["pico", "nano", "micro", "milli", "kilo"],
            ),
            (
                UnitFamilyId::Mass,
                "gram",
                &["pico", "nano", "micro", "milli", "kilo", "mega"],
            ),
            (
                UnitFamilyId::Time,
                "second",
                &["pico", "nano", "micro", "milli", "kilo"],
            ),
            (
                UnitFamilyId::Current,
                "ampere",
                &["nano", "micro", "milli", "kilo", "mega"],
            ),
            (
                UnitFamilyId::Temperature,
                "kelvin",
                &["nano", "micro", "milli", "kilo"],
            ),
            (
                UnitFamilyId::Amount,
                "mole",
                &["pico", "nano", "micro", "milli", "kilo"],
            ),
        ];

        for (family, name, expected) in cases {
            let family_key = format!("u_{}", family.description().to_lowercase());
            for prefix in prefixes {
                let key = format!("{family_key}_{prefix}{name}");
                let unit = UnitId::from_unit_id_str(&key);
                assert_eq!(
                    unit.is_some(),
                    expected.contains(&prefix),
                    "unexpected prefix coverage for {key}"
                );
                if let Some(unit) = unit {
                    assert_eq!(unit.family_id(), *family);
                    assert!(family.unit_ids().contains(&unit));
                }
            }
        }
    }

    #[test]
    fn unit_family_id_from_u8_returns_none_for_invalid_values() {
        let invalid_values = [52, 53, 55, 56, 255];
        for &value in &invalid_values {
            assert_eq!(
                UnitFamilyId::from_u8(value),
                None,
                "from_u8({value}) should return None",
            );
        }
    }

    #[test]
    fn families_list_metric_variants_before_other_units() {
        let cases: &[(UnitFamilyId, &[&str])] = &[
            (UnitFamilyId::Current, &["A", "nA", "μA", "mA", "kA", "MA"]),
            (
                UnitFamilyId::Temperature,
                &["K", "nK", "μK", "mK", "kK", "°C", "°F"],
            ),
            (
                UnitFamilyId::Angle,
                &[
                    "rad", "prad", "nrad", "μrad", "mrad", "krad", "Mrad", "Grad", "Trad", "°",
                    "rev", "′", "″",
                ],
            ),
            (
                UnitFamilyId::Area,
                &[
                    "m²", "μm²", "mm²", "cm²", "km²", "ft²", "in²", "ac", "ha", "mi²",
                ],
            ),
            (
                UnitFamilyId::Volume,
                &[
                    "m³",
                    "cm³",
                    "mm³",
                    "L",
                    "pL",
                    "nL",
                    "μL",
                    "mL",
                    "kL",
                    "ML",
                    "gal",
                    "imp gal",
                    "fl oz",
                    "imp fl oz",
                    "cup",
                    "pt",
                    "qt",
                ],
            ),
            (
                UnitFamilyId::Speed,
                &["m/s", "mm/s", "cm/s", "km/h", "in/s", "ft/s", "mph", "kn"],
            ),
            (
                UnitFamilyId::Acceleration,
                &["m/s²", "mm/s²", "cm/s²", "in/s²", "ft/s²", "g₀"],
            ),
            (
                UnitFamilyId::AngularVelocity,
                &["rad/s", "°/s", "rpm", "rev/s"],
            ),
            (
                UnitFamilyId::Torque,
                &["N·m", "N·mm", "N·cm", "lbf·in", "lbf·ft"],
            ),
            (
                UnitFamilyId::Density,
                &["kg/m³", "g/cm³", "g/L", "kg/L", "lb/ft³"],
            ),
            (
                UnitFamilyId::AngularFrequency,
                &[
                    "rad/s", "prad/s", "nrad/s", "μrad/s", "mrad/s", "krad/s", "Mrad/s", "Grad/s",
                    "Trad/s",
                ],
            ),
        ];

        for (family, expected) in cases {
            let descriptions: Vec<_> = family.unit_ids().iter().map(UnitId::description).collect();
            assert_eq!(descriptions, *expected, "unexpected order for {family:?}");
        }
    }

    #[test]
    fn unit_from_invalid_u16_returns_none() {
        let invalid_values = [
            99, 111, 2210, 2317, 2805, 2902, 3002, 3109, 4202, 5102, 65535,
        ];
        for &value in &invalid_values {
            assert_eq!(
                UnitId::from_u16(value),
                None,
                "from_u16({value}) should return None",
            );
        }
    }

    #[test]
    fn removed_specialized_unit_keys_are_not_registered() {
        let removed_units = [
            ("luminous_intensity", "candela"),
            ("luminous_flux", "lumen"),
            ("illuminance", "lux"),
            ("radioactivity", "becquerel"),
            ("absorbed_dose", "gray"),
            ("equivalent_dose", "sievert"),
            ("catalytic_activity", "katal"),
        ];
        for (family, name) in removed_units {
            for prefix in [
                "", "pico", "nano", "micro", "milli", "kilo", "mega", "giga", "tera",
            ] {
                let key = format!("u_{family}_{prefix}{name}");
                assert_eq!(UnitId::from_unit_id_str(&key), None);
            }
        }
        for key in [
            "u_luminous_intensity_hefnerkerze",
            "u_luminous_intensity_international_candle",
            "u_luminous_intensity_decimal_candle",
        ] {
            assert_eq!(UnitId::from_unit_id_str(key), None);
        }
    }

    #[test]
    fn named_si_derived_units_are_registered() {
        let units = [
            ("u_temperature_celsius", 505, "°C"),
            ("u_frequency_hertz", 700, "Hz"),
            ("u_angle_radian", 800, "rad"),
            ("u_solid_angle_steradian", 900, "sr"),
            ("u_force_newton", 1000, "N"),
            ("u_pressure_pascal", 1100, "Pa"),
            ("u_energy_joule", 1200, "J"),
            ("u_power_watt", 1300, "W"),
            ("u_electric_charge_coulomb", 1400, "C"),
            ("u_voltage_volt", 1500, "V"),
            ("u_resistance_ohm", 1600, "Ω"),
            ("u_conductance_siemens", 1700, "S"),
            ("u_capacitance_farad", 1800, "F"),
            ("u_inductance_henry", 1900, "H"),
            ("u_magnetic_flux_density_tesla", 2000, "T"),
            ("u_magnetic_flux_weber", 2100, "Wb"),
        ];

        for (key, value, symbol) in units {
            let unit = UnitId::from_unit_id_str(key);
            assert!(unit.is_some(), "missing unit: {key}");
            assert_eq!(unit, UnitId::from_u16(value));
            if let Some(unit) = unit {
                assert_eq!(unit.description(), symbol);
                assert_eq!(unit.string_id().to_string(), key);
            }
        }
    }

    #[test]
    fn degree_is_registered_in_the_angle_family() {
        let degree = UnitId::Angle_Degree;
        assert_eq!(UnitId::from_unit_id_str("u_angle_degree"), Some(degree));
        assert_eq!(UnitId::from_u16(809), Some(degree));
        assert_eq!(degree.string_id().to_string(), "u_angle_degree");
        assert_eq!(degree.description(), "°");
        assert_eq!(degree.family_id(), UnitFamilyId::Angle);
        assert!(UnitFamilyId::Angle.unit_ids().contains(&degree));
    }

    #[test]
    fn derived_families_have_supported_engineering_prefixes() {
        let prefixes = [
            ("pico", "p"),
            ("nano", "n"),
            ("micro", "μ"),
            ("milli", "m"),
            ("kilo", "k"),
            ("mega", "M"),
            ("giga", "G"),
            ("tera", "T"),
        ];

        for family in UnitFamilyId::ALL {
            if !(UnitFamilyId::Frequency.to_u8()..=UnitFamilyId::MagneticFlux.to_u8())
                .contains(&family.to_u8())
            {
                continue;
            }

            let Some(base) = family.unit_ids().first() else {
                panic!("missing base unit for {family:?}");
            };
            let base_key = base.string_id();
            let Some((family_key, name)) = base_key.as_str().rsplit_once('_') else {
                panic!("invalid base unit key: {base_key}");
            };

            let prefix_count = if matches!(
                family,
                UnitFamilyId::SolidAngle
                    | UnitFamilyId::Inductance
                    | UnitFamilyId::MagneticFluxDensity
            ) {
                4
            } else if family == UnitFamilyId::Capacitance {
                5
            } else {
                prefixes.len()
            };

            for (index, (prefix, symbol)) in prefixes.iter().enumerate() {
                let key = format!("{family_key}_{prefix}{name}");
                if index >= prefix_count {
                    assert_eq!(UnitId::from_unit_id_str(&key), None);
                    let Ok(offset) = u16::try_from(index + 1) else {
                        panic!("prefix offset must fit in u16: {}", index + 1);
                    };
                    let value = u16::from(family.to_u8()) * 100 + offset;
                    assert_eq!(UnitId::from_u16(value), None);
                    continue;
                }
                let Some(unit) = UnitId::from_unit_id_str(&key) else {
                    panic!("missing prefixed unit: {key}");
                };
                assert_eq!(unit.string_id().to_string(), key);
                assert_eq!(
                    unit.description(),
                    format!("{symbol}{}", base.description())
                );
                assert_eq!(unit.family_id(), family);
                assert!(family.unit_ids().contains(&unit));
            }
        }

        assert_eq!(UnitId::Voltage_Millivolt.description(), "mV");
        assert_eq!(UnitId::Voltage_Megavolt.description(), "MV");
    }

    #[test]
    fn numeric_ids_and_registration_order_match_display_order() {
        let mut ordered_units = Vec::new();

        for (family_index, family) in UnitFamilyId::ALL.iter().enumerate() {
            let Ok(family_value) = u8::try_from(family_index) else {
                panic!("family index must fit in u8: {family_index}");
            };
            assert_eq!(family.to_u8(), family_value);
            assert_eq!(UnitFamilyId::from_u8(family_value), Some(*family));

            for (unit_index, unit) in family.unit_ids().iter().enumerate() {
                let Ok(unit_offset) = u16::try_from(unit_index) else {
                    panic!("unit index must fit in u16: {unit_index}");
                };
                let unit_value = u16::from(family_value) * 100 + unit_offset;
                assert_eq!(unit.to_u16(), unit_value, "unexpected ID for {unit:?}");
                assert_eq!(UnitId::from_u16(unit_value), Some(*unit));
                ordered_units.push(*unit);
            }
        }

        assert_eq!(UnitId::ALL, ordered_units.as_slice());
    }

    #[test]
    fn unit_definitions_are_consistent() {
        let mut unit_values = HashSet::new();
        let mut unit_string_ids = HashSet::new();

        for unit_id in UnitId::ALL {
            let value = unit_id.to_u16();
            let family = unit_id.family_id();
            let key = unit_id.string_id().to_string();

            assert!(unit_values.insert(value), "duplicate unit value: {value}");
            assert!(
                unit_string_ids.insert(key.clone()),
                "duplicate unit key: {key}"
            );
            assert_eq!(UnitId::from_u16(value), Some(*unit_id));
            assert_eq!(UnitId::from_unit_id_str(&key), Some(*unit_id));
            assert_eq!(value / 100, u16::from(family.to_u8()));
            assert!(family.unit_ids().contains(unit_id));
            if *unit_id != UnitId::None {
                assert_ne!(unit_id.description(), "");
            }
            assert_ne!(unit_id.documentation(), "");
            assert_ne!(unit_id.documentation(), unit_id.description());
            assert!(!unit_id.description().contains(['(', ')']));
        }

        for family in UnitFamilyId::ALL {
            let family_unit_ids = family.unit_ids();
            let family_units = UnitId::ALL
                .iter()
                .filter(|unit_id| unit_id.family_id() == family)
                .count();

            assert_eq!(UnitFamilyId::from_u8(family.to_u8()), Some(family));
            assert_eq!(family_unit_ids.len(), family_units);

            for unit_id in family_unit_ids {
                assert_eq!(
                    unit_id.family_id(),
                    family,
                    "family unit must belong to its family: {unit_id:?}"
                );
            }
        }
    }
}

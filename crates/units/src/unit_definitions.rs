use datastore::key::ConstUnitKey;
use datastore::unit_key;
use serde::{Deserialize, Serialize};

/// `UnitFamilyId` is an enum that represents the different families of units.
/// Each family has a unique identifier that can be used to group units together.
/// The `UnitFamilyId` enum is used in the Unit struct to specify the family of a unit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum UnitFamilyId {
    /// The family of units that do not belong to any specific category.
    None = 0,
    /// The family of units that measure area.
    Area = 1,
    /// The family of units that measure electric current.
    Current = 2,
    /// The family of units that measure length.
    Length = 3,
    /// The family of units that measure luminous intensity.
    Luminosity = 4,
    /// The family of units that measure the amount of substance.
    Amount = 5,
    /// The family of units that measure temperature.
    Temperature = 6,
    /// The family of units that measure time.
    Time = 7,
    /// The family of units that measure volume.
    Volume = 8,
    /// The family of units that measure weight.
    Weight = 9,
}

impl UnitFamilyId {
    /// All supported unit families.
    pub const ALL: [Self; 10] = [
        Self::None,
        Self::Area,
        Self::Current,
        Self::Length,
        Self::Luminosity,
        Self::Amount,
        Self::Temperature,
        Self::Time,
        Self::Volume,
        Self::Weight,
    ];

    /// Returns the `UnitFamilyId` corresponding to the given u8 value.
    #[must_use]
    pub const fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(UnitFamilyId::None),
            1 => Some(UnitFamilyId::Area),
            2 => Some(UnitFamilyId::Current),
            3 => Some(UnitFamilyId::Length),
            4 => Some(UnitFamilyId::Luminosity),
            5 => Some(UnitFamilyId::Amount),
            6 => Some(UnitFamilyId::Temperature),
            7 => Some(UnitFamilyId::Time),
            8 => Some(UnitFamilyId::Volume),
            9 => Some(UnitFamilyId::Weight),
            _ => None,
        }
    }

    /// Returns the u8 value corresponding to the given `UnitFamilyId`.
    #[must_use]
    pub const fn to_u8(&self) -> u8 {
        match self {
            Self::None => 0,
            Self::Area => 1,
            Self::Current => 2,
            Self::Length => 3,
            Self::Luminosity => 4,
            Self::Amount => 5,
            Self::Temperature => 6,
            Self::Time => 7,
            Self::Volume => 8,
            Self::Weight => 9,
        }
    }

    /// Returns the display name for this family.
    #[must_use]
    pub const fn description(&self) -> &'static str {
        match self {
            UnitFamilyId::None => "None",
            UnitFamilyId::Area => "Area",
            UnitFamilyId::Current => "Current",
            UnitFamilyId::Length => "Length",
            UnitFamilyId::Luminosity => "Luminosity",
            UnitFamilyId::Amount => "Amount",
            UnitFamilyId::Temperature => "Temperature",
            UnitFamilyId::Time => "Time",
            UnitFamilyId::Volume => "Volume",
            UnitFamilyId::Weight => "Weight",
        }
    }
    /// Returns the unit identifiers for all units in this family.
    #[must_use]
    pub const fn unit_ids(&self) -> &[UnitId] {
        match self {
            UnitFamilyId::None => &[UnitId::None],
            UnitFamilyId::Area => &[
                UnitId::Area_SquareMeter,
                UnitId::Area_SquareMilliMeter,
                UnitId::Area_SquareCentiMeter,
                UnitId::Area_SquareKiloMeter,
                UnitId::Area_SquareFoot,
                UnitId::Area_SquareInch,
                UnitId::Area_Acre,
                UnitId::Area_Hectare,
                UnitId::Area_SquareMile,
            ],
            UnitFamilyId::Current => &[
                UnitId::Current_Ampere,
                UnitId::Current_Milliampere,
                UnitId::Current_Microampere,
                UnitId::Current_Nanoampere,
                UnitId::Current_Kiloampere,
            ],
            UnitFamilyId::Length => &[
                UnitId::Length_Meter,
                UnitId::Length_Kilometer,
                UnitId::Length_Centimeter,
                UnitId::Length_Millimeter,
                UnitId::Length_Foot,
                UnitId::Length_Inch,
                UnitId::Length_Yard,
                UnitId::Length_Mile,
            ],
            UnitFamilyId::Luminosity => &[
                UnitId::Luminosity_Candela,
                UnitId::Luminosity_Millicandela,
                UnitId::Luminosity_Kilocandela,
                UnitId::Luminosity_Hefnerkerze,
                UnitId::Luminosity_InternationalCandle,
                UnitId::Luminosity_DecimalCandle,
            ],
            UnitFamilyId::Amount => &[
                UnitId::Amount_Mole,
                UnitId::Amount_Millimole,
                UnitId::Amount_Micromole,
                UnitId::Amount_Nanomole,
                UnitId::Amount_Picomole,
                UnitId::Amount_Kilomole,
            ],
            UnitFamilyId::Temperature => &[
                UnitId::Temperature_Celsius,
                UnitId::Temperature_Fahrenheit,
                UnitId::Temperature_Kelvin,
            ],
            UnitFamilyId::Time => &[
                UnitId::Time_Second,
                UnitId::Time_Minute,
                UnitId::Time_Hour,
                UnitId::Time_Day,
                UnitId::Time_Week,
                UnitId::Time_Year,
                UnitId::Time_MilliSecond,
                UnitId::Time_MicroSecond,
                UnitId::Time_NanoSecond,
            ],
            UnitFamilyId::Volume => &[
                UnitId::Volume_Liter,
                UnitId::Volume_Milliliter,
                UnitId::Volume_Gallon,
                UnitId::Volume_ImperialGallon,
                UnitId::Volume_FluidOunce,
                UnitId::Volume_ImperialFluidOunce,
                UnitId::Volume_Cup,
                UnitId::Volume_Pint,
                UnitId::Volume_Quart,
            ],
            UnitFamilyId::Weight => &[
                UnitId::Weight_Kilogram,
                UnitId::Weight_Gram,
                UnitId::Weight_Pound,
                UnitId::Weight_Ounce,
                UnitId::Weight_Tonne,
                UnitId::Weight_Stone,
            ],
        }
    }
}

/// Defines [`UnitId`] variants and their associated metadata.
macro_rules! define_unit_ids {
    ($(
        $unit:ident = $value:expr => (
            $family:ident,
            $key:literal,
            $description:literal,
            $documentation:literal
        ),
    )*) => {
        /// Identifiers for the units supported by the calculator.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
        #[repr(u16)]
        #[allow(non_camel_case_types)]
        pub enum UnitId {
            $(
                #[doc = $documentation]
                $unit = $value,
            )*
        }

        impl UnitId {
            /// All supported unit identifiers.
            pub const ALL: &[Self] = &[
                $(
                    Self::$unit,
                )*
            ];

            /// Returns the `UnitId` corresponding to the given string identifier.
            pub fn from_unit_id_str(unit_id_str: &str) -> Option<Self> {
                match unit_id_str {
                    $(
                        $key => Some(Self::$unit),
                    )*
                    _ => None,
                }
            }

            /// Returns the `UnitId` corresponding to the given u16 value.
            pub const fn from_u16(value: u16) -> Option<Self> {
                match value {
                    $(
                        $value => Some(Self::$unit),
                    )*
                    _ => None,
                }
            }

            /// Returns the u16 value corresponding to the given `UnitId`.
            pub const fn to_u16(&self) -> u16 {
                match self {
                    $(
                        Self::$unit => $value,
                    )*
                }
            }

            /// Returns the `UnitFamilyId` corresponding to the given `UnitId`.
            pub const fn family_id(&self) -> UnitFamilyId {
                match self {
                    $(
                        Self::$unit => UnitFamilyId::$family,
                    )*
                }
            }

            /// Returns the string identifier corresponding to the given `UnitId`.
            pub const fn string_id(&self) -> ConstUnitKey {
                match self {
                    $(
                        Self::$unit => unit_key!($key),
                    )*
                }
            }

            /// Returns the description corresponding to the given `UnitId`.
            pub const fn description(&self) -> &'static str {
                match self {
                    $(
                        Self::$unit => $description,
                    )*
                }
            }

            /// Returns the documentation corresponding to the given `UnitId`.
            #[allow(dead_code, reason = "This is to check that the documentation is not empty for each unit.")]
            const fn documentation(&self) -> &'static str {
                match self {
                    $(
                        Self::$unit => $documentation,
                    )*
                }
            }
        }
    };
}

define_unit_ids! {
    None = 0 => (None, "u_none", "", "No unit."),

    Area_SquareMeter = 100 => (Area, "u_area_square_meter", "m²", "A square meter."),
    Area_SquareMilliMeter = 101 => (Area, "u_area_square_millimeter", "mm²", "A square millimeter."),
    Area_SquareCentiMeter = 102 => (Area, "u_area_square_centimeter", "cm²", "A square centimeter."),
    Area_SquareKiloMeter = 103 => (Area, "u_area_square_kilometer", "km²", "A square kilometer."),
    Area_SquareFoot = 104 => (Area, "u_area_square_foot", "ft²", "A square foot."),
    Area_SquareInch = 105 => (Area, "u_area_square_inch", "in²", "A square inch."),
    Area_Acre = 106 => (Area, "u_area_acre", "ac", "An acre."),
    Area_Hectare = 107 => (Area, "u_area_hectare", "ha", "A hectare."),
    Area_SquareMile = 108 => (Area, "u_area_square_mile", "mi²", "A square mile."),

    Current_Ampere = 200 => (Current, "u_current_ampere", "A", "An ampere."),
    Current_Milliampere = 201 => (Current, "u_current_milliampere", "mA", "A milliampere."),
    Current_Microampere = 202 => (Current, "u_current_microampere", "μA", "A microampere."),
    Current_Nanoampere = 203 => (Current, "u_current_nanoampere", "nA", "A nanoampere."),
    Current_Kiloampere = 204 => (Current, "u_current_kiloampere", "kA", "A kiloampere."),

    Length_Meter = 300 => (Length, "u_length_meter", "m", "A meter."),
    Length_Kilometer = 301 => (Length, "u_length_kilometer", "km", "A kilometer."),
    Length_Centimeter = 302 => (Length, "u_length_centimeter", "cm", "A centimeter."),
    Length_Millimeter = 303 => (Length, "u_length_millimeter", "mm", "A millimeter."),
    Length_Foot = 304 => (Length, "u_length_foot", "ft", "A foot."),
    Length_Inch = 305 => (Length, "u_length_inch", "in", "An inch."),
    Length_Yard = 306 => (Length, "u_length_yard", "yd", "A yard."),
    Length_Mile = 307 => (Length, "u_length_mile", "mi", "A mile."),

    Luminosity_Candela = 400 => (Luminosity, "u_luminosity_candela", "cd", "A candela."),
    Luminosity_Millicandela = 401 => (Luminosity, "u_luminosity_millicandela", "mcd", "A millicandela."),
    Luminosity_Kilocandela = 402 => (Luminosity, "u_luminosity_kilocandela", "kcd", "A kilocandela."),
    Luminosity_Hefnerkerze = 403 => (Luminosity, "u_luminosity_hefnerkerze", "hk", "A Hefnerkerze."),
    Luminosity_InternationalCandle = 404 => (Luminosity, "u_luminosity_international_candle", "ic", "An international candle."),
    Luminosity_DecimalCandle = 405 => (Luminosity, "u_luminosity_decimal_candle", "dc", "A decimal candle."),

    Amount_Mole = 500 => (Amount, "u_amount_mole", "mol", "A mole."),
    Amount_Millimole = 501 => (Amount, "u_amount_millimole", "mmol", "A millimole."),
    Amount_Micromole = 502 => (Amount, "u_amount_micromole", "μmol", "A micromole."),
    Amount_Nanomole = 503 => (Amount, "u_amount_nanomole", "nmol", "A nanomole."),
    Amount_Picomole = 504 => (Amount, "u_amount_picomole", "pmol", "A picomole."),
    Amount_Kilomole = 505 => (Amount, "u_amount_kilomole", "kmol", "A kilomole."),

    Temperature_Celsius = 600 => (Temperature, "u_temperature_celsius", "°C", "A degree Celsius."),
    Temperature_Fahrenheit = 601 => (Temperature, "u_temperature_fahrenheit", "°F", "A degree Fahrenheit."),
    Temperature_Kelvin = 602 => (Temperature, "u_temperature_kelvin", "K", "A kelvin."),

    Time_Second = 700 => (Time, "u_time_second", "s", "A second."),
    Time_Minute = 701 => (Time, "u_time_minute", "min", "A minute."),
    Time_Hour = 702 => (Time, "u_time_hour", "h", "An hour."),
    Time_Day = 703 => (Time, "u_time_day", "day", "A day."),
    Time_Week = 704 => (Time, "u_time_week", "week", "A week."),
    Time_Year = 705 => (Time, "u_time_year", "year", "A year."),
    Time_MilliSecond = 706 => (Time, "u_time_millisecond", "ms", "A millisecond."),
    Time_MicroSecond = 707 => (Time, "u_time_microsecond", "μs", "A microsecond."),
    Time_NanoSecond = 708 => (Time, "u_time_nanosecond", "ns", "A nanosecond."),

    Volume_Liter = 800 => (Volume, "u_volume_liter", "l", "A liter."),
    Volume_Milliliter = 801 => (Volume, "u_volume_milliliter", "ml", "A milliliter."),
    Volume_Gallon = 802 => (Volume, "u_volume_gallon", "gal", "A US gallon."),
    Volume_ImperialGallon = 803 => (Volume, "u_volume_imperial_gallon", "gal_uk", "An imperial gallon."),
    Volume_FluidOunce = 804 => (Volume, "u_volume_fluid_ounce", "fl_oz", "A US fluid ounce."),
    Volume_ImperialFluidOunce = 805 => (Volume, "u_volume_imperial_fluid_ounce", "fl_oz_uk", "An imperial fluid ounce."),
    Volume_Cup = 806 => (Volume, "u_volume_cup", "cup", "A US customary cup."),
    Volume_Pint = 807 => (Volume, "u_volume_pint", "pt", "A US pint."),
    Volume_Quart = 808 => (Volume, "u_volume_quart", "qt", "A US quart."),

    Weight_Kilogram = 900 => (Weight, "u_weight_kilogram", "kg", "A kilogram."),
    Weight_Gram = 901 => (Weight, "u_weight_gram", "g", "A gram."),
    Weight_Pound = 902 => (Weight, "u_weight_pound", "lb", "A pound."),
    Weight_Ounce = 903 => (Weight, "u_weight_ounce", "oz", "An ounce."),
    Weight_Tonne = 904 => (Weight, "u_weight_tonne", "t", "A metric tonne."),
    Weight_Stone = 905 => (Weight, "u_weight_stone", "st", "A stone."),
}

#[cfg(test)]
mod tests {
    use super::{UnitFamilyId, UnitId};
    use std::collections::HashSet;

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
            assert!(!family.description().is_empty());
            assert!(!family.description().contains(['(', ')']));

            let description = match family {
                UnitFamilyId::None => "None",
                UnitFamilyId::Area => "Area",
                UnitFamilyId::Current => "Current",
                UnitFamilyId::Length => "Length",
                UnitFamilyId::Luminosity => "Luminosity",
                UnitFamilyId::Amount => "Amount",
                UnitFamilyId::Temperature => "Temperature",
                UnitFamilyId::Time => "Time",
                UnitFamilyId::Volume => "Volume",
                UnitFamilyId::Weight => "Weight",
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
    fn unit_family_id_from_u8_returns_none_for_invalid_values() {
        let invalid_values = [10, 255];
        for &value in &invalid_values {
            assert_eq!(
                UnitFamilyId::from_u8(value),
                None,
                "from_u8({value}) should return None",
            );
        }
    }

    #[test]
    fn unit_from_invalid_u16_returns_none() {
        let invalid_values = [999, 1000, 65535];
        for &value in &invalid_values {
            assert_eq!(
                UnitId::from_u16(value),
                None,
                "from_u16({value}) should return None",
            );
        }
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
            assert_eq!(value / 100, u16::from(family.to_u8()));
            assert!(family.unit_ids().contains(unit_id));
            if *unit_id != UnitId::None {
                assert!(!unit_id.description().is_empty());
            }
            assert!(!unit_id.documentation().is_empty());
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

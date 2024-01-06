use fake::{
    faker::internet::en::FreeEmail,
    faker::{
        address::en::{CityName, CountryCode, CountryName, StateName, StreetName, PostCode},
        name::raw::*,
        phone_number::en::CellNumber,
    },
    uuid::{UUIDv1, UUIDv4, UUIDv3},
};
use fake::{locales::*, Fake};

pub struct Parser;

impl Parser {
    pub fn parse(role: &str) -> String {
        match role {
            "${uuidV1}" => UUIDv1.fake(),
            "${uuidV3}" => UUIDv3.fake(),
            "${uuidV4}" => UUIDv4.fake(),
            "${name}" => Name(EN).fake(),
            "${email}" => FreeEmail().fake(),
            "${phone}" => CellNumber().fake(),
            "${city}" => CityName().fake(),
            "${countryCode}" => CountryCode().fake(),
            "${countryName}" => CountryName().fake(),
            "${stateName}" => StateName().fake(),
            "${streetName}" => StreetName().fake(),
            "${zipCode}" => StreetName().fake(),
            "${postCode}" => PostCode().fake(),
            _ => String::from(""),
        }
    }
}

use json::{object, JsonValue};
use struct_iterable::Iterable;

use super::model::*;

/// A Trait that states that 'Self' can be made into a `Structure`
pub trait ToStructure {
    /// A function for annotation name of fields in the structure with their description
    fn descriptions() -> JsonValue;
    /// A function which processes the struct and returns a JsonValue of the name and value of the fields
    fn structure_items(&self) -> Vec<StructureItem>;
    /// A function used to make the substructures, if any, of the struct
    fn substructures(&self) -> Option<Vec<Structure>>;
}

impl ToStructure for Fib {
    fn descriptions() -> JsonValue {
        object! {
            fcMin: "The Min value of something",
        }
    }

    fn structure_items(&self) -> Vec<StructureItem> {
        let descriptions = Self::descriptions();
        let self_json = json::parse(&serde_json::to_string(&self).unwrap()).unwrap();
        let mut structure_items = vec![];
        for (field_name, _) in self.iter() {
            let field_val = self_json[field_name].to_string();
            let description = if descriptions.has_key(&field_name) {
                Some(descriptions[field_name].clone().to_string())
            } else {
                None
            };

            structure_items.push(StructureItem {
                name: field_name.to_string(),
                value: field_val,
                description,
            });
        }

        structure_items
    }

    fn substructures(&self) -> Option<Vec<Structure>> {
        None
    }
}

impl ToStructure for SHSHI {
    fn descriptions() -> JsonValue {
        object! {}
    }

    fn structure_items(&self) -> Vec<StructureItem> {
        let descriptions = Self::descriptions();
        let self_json = json::parse(&serde_json::to_string(&self).unwrap()).unwrap();

        let mut structure_items = vec![];
        for (field_name, _) in self.iter() {
            // Ignoreing Styles as it is a substructure
            if field_name == "styles" {
                continue;
            }

            let field_val = self_json[field_name].to_string();
            let description = if descriptions.has_key(&field_name) {
                Some(descriptions[field_name].clone().to_string())
            } else {
                None
            };

            structure_items.push(StructureItem {
                name: field_name.to_string(),
                value: field_val,
                description,
            });
        }

        structure_items
    }

    fn substructures(&self) -> Option<Vec<Structure>> {
        let mut substructures = vec![];

        for style in &self.styles {
            let style_structure = Structure::from(&style.xstzName, style);
            substructures.push(style_structure);
        }

        Some(substructures)
    }
}

impl ToStructure for STD {
    fn descriptions() -> JsonValue {
        object! {
            sti: "The style identifier",
        }
    }

    fn structure_items(&self) -> Vec<StructureItem> {
        let descriptions = Self::descriptions();
        let self_json = json::parse(&serde_json::to_string(&self).unwrap()).unwrap();
        let mut structure_items = vec![];
        for (field_name, _) in self.iter() {
            let field_val = self_json[field_name].to_string();
            let description = if descriptions.has_key(&field_name) {
                Some(descriptions[field_name].clone().to_string())
            } else {
                None
            };

            structure_items.push(StructureItem {
                name: field_name.to_string(),
                value: field_val,
                description,
            });
        }

        structure_items
    }

    fn substructures(&self) -> Option<Vec<Structure>> {
        None
    }
}

impl ToStructure for DocumentSummaryInfoStream {
    fn descriptions() -> JsonValue {
        object! {}
    }

    fn structure_items(&self) -> Vec<StructureItem> {
        let descriptions = Self::descriptions();
        let self_json = json::parse(&serde_json::to_string(&self).unwrap()).unwrap();
        let mut structure_items = vec![];
        for (field_name, _) in self.iter() {
            let field_val = self_json[field_name].to_string();
            let description = if descriptions.has_key(&field_name) {
                Some(descriptions[field_name].clone().to_string())
            } else {
                None
            };

            structure_items.push(StructureItem {
                name: field_name.to_string(),
                value: field_val,
                description,
            });
        }

        structure_items    
    }

    fn substructures(&self) -> Option<Vec<Structure>> {
        None
    }
}

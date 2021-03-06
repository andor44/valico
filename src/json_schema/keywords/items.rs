use rustc_serialize::json;

use super::super::schema;
use super::super::validators;
use super::super::helpers;

#[allow(missing_copy_implementations)]
pub struct Items;
impl super::Keyword for Items {
    fn compile(&self, def: &json::Json, ctx: &schema::WalkContext) -> super::KeywordResult {
        let maybe_items = def.find("items");
        let maybe_additional = def.find("additionalItems");

        if !(maybe_items.is_some() || maybe_additional.is_some()) {
            return Ok(None)
        }

        let items = if maybe_items.is_some() {
            let items_val = maybe_items.unwrap();
            Some(if items_val.is_object() {

                validators::items::ItemsKind::Schema(
                    helpers::alter_fragment_path(ctx.url.clone(), [
                        ctx.escaped_fragment().as_ref(),
                        "items"
                    ].connect("/"))
                )

            } else if items_val.is_array() {

                let mut schemas = vec![];
                for (idx, item) in items_val.as_array().unwrap().iter().enumerate() {
                    if item.is_object() {
                        schemas.push(
                            helpers::alter_fragment_path(ctx.url.clone(), [
                                ctx.escaped_fragment().as_ref(),
                                "items",
                                idx.to_string().as_ref()
                            ].connect("/"))
                        )
                    } else {
                        return Err(schema::SchemaError::Malformed {
                            path: ctx.fragment.connect("/"),
                            detail: "Items of this array MUST be objects".to_string()
                        })
                    }
                }

                validators::items::ItemsKind::Array(schemas)

            } else {

                return Err(schema::SchemaError::Malformed {
                    path: ctx.fragment.connect("/"),
                    detail: "`items` must be an object or an array".to_string()
                })

            })
        } else {
            None
        };

        let additional_items = if maybe_additional.is_some() {
            let additional_val = maybe_additional.unwrap();
            Some(if additional_val.is_boolean() {

                validators::items::AdditionalKind::Boolean(additional_val.as_boolean().unwrap())

            } else if additional_val.is_object() {

                validators::items::AdditionalKind::Schema(
                    helpers::alter_fragment_path(ctx.url.clone(), [
                        ctx.escaped_fragment().as_ref(),
                        "additionalItems"
                    ].connect("/"))
                )

            } else {

                return Err(schema::SchemaError::Malformed {
                    path: ctx.fragment.connect("/"),
                    detail: "`additionalItems` must be a boolean or an object".to_string()
                })

            })
        } else {
            None
        };

        Ok(Some(Box::new(validators::Items {
            items: items,
            additional: additional_items
        })))

    }
}

#[cfg(test)] use super::super::scope;
#[cfg(test)] use super::super::builder;
#[cfg(test)] use rustc_serialize::json::{ToJson};

#[test]
fn validate_items_with_schema() {
    let mut scope = scope::Scope::new();
    let schema = scope.compile_and_return(builder::schema(|s| {
        s.items_schema(|items| {
            items.minimum(5f64, false);
            items.maximum(10f64, false);
        });
    }).into_json(), true).ok().unwrap();

    assert_eq!(schema.validate(&[5,6,7,8,9,10].to_json()).is_valid(), true);
    assert_eq!(schema.validate(&[4,5,6,7,8,9,10].to_json()).is_valid(), false);
    assert_eq!(schema.validate(&[5,6,7,8,9,10,11].to_json()).is_valid(), false);
}

#[test]
fn validate_items_with_array_of_schemes() {
    let mut scope = scope::Scope::new();
    let schema = scope.compile_and_return(builder::schema(|s| {
        s.items_array(|items| {
            items.push(|item| {
                item.minimum(1f64, false);
                item.maximum(3f64, false);
            });
            items.push(|item| {
                item.minimum(3f64, false);
                item.maximum(6f64, false);
            });
        })
    }).into_json(), true).ok().unwrap();

    assert_eq!(schema.validate(&[1].to_json()).is_valid(), true);
    assert_eq!(schema.validate(&[1,3].to_json()).is_valid(), true);
    assert_eq!(schema.validate(&[1,3,100].to_json()).is_valid(), true);
    assert_eq!(schema.validate(&[4,3].to_json()).is_valid(), false);
    assert_eq!(schema.validate(&[1,7].to_json()).is_valid(), false);
    assert_eq!(schema.validate(&[4,7].to_json()).is_valid(), false);
}

#[test]
fn validate_items_with_array_of_schemes_with_additional_bool() {
    let mut scope = scope::Scope::new();
    let schema = scope.compile_and_return(builder::schema(|s| {
        s.items_array(|items| {
            items.push(|item| {
                item.minimum(1f64, false);
                item.maximum(3f64, false);
            });
            items.push(|item| {
                item.minimum(3f64, false);
                item.maximum(6f64, false);
            });
        });
        s.additional_items(false);
    }).into_json(), true).ok().unwrap();

    assert_eq!(schema.validate(&[1,3,100].to_json()).is_valid(), false);
}

#[test]
fn validate_items_with_array_of_schemes_with_additional_schema() {
    let mut scope = scope::Scope::new();
    let schema = scope.compile_and_return(builder::schema(|s| {
        s.items_array(|items| {
            items.push(|item| {
                item.minimum(1f64, false);
                item.maximum(3f64, false);
            });
            items.push(|item| {
                item.minimum(3f64, false);
                item.maximum(6f64, false);
            });
        });
        s.additional_items_schema(|add| {
           add.maximum(100f64, false)
        });
    }).into_json(), true).ok().unwrap();

    assert_eq!(schema.validate(&[1,3,100].to_json()).is_valid(), true);
    assert_eq!(schema.validate(&[1,3,101].to_json()).is_valid(), false);
}
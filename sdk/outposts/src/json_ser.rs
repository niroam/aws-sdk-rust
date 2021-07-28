// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_create_outpost_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateOutpostInput,
) {
    if let Some(var_1) = &input.availability_zone {
        object.key("AvailabilityZone").string(var_1);
    }
    if let Some(var_2) = &input.availability_zone_id {
        object.key("AvailabilityZoneId").string(var_2);
    }
    if let Some(var_3) = &input.description {
        object.key("Description").string(var_3);
    }
    if let Some(var_4) = &input.name {
        object.key("Name").string(var_4);
    }
    if let Some(var_5) = &input.site_id {
        object.key("SiteId").string(var_5);
    }
    if let Some(var_6) = &input.tags {
        let mut object_7 = object.key("Tags").start_object();
        for (key_8, value_9) in var_6 {
            {
                object_7.key(key_8).string(value_9);
            }
        }
        object_7.finish();
    }
}

pub fn serialize_structure_tag_resource_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) {
    if let Some(var_10) = &input.tags {
        let mut object_11 = object.key("Tags").start_object();
        for (key_12, value_13) in var_10 {
            {
                object_11.key(key_12).string(value_13);
            }
        }
        object_11.finish();
    }
}
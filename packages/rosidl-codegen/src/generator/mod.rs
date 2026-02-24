mod action;
mod common;
mod msg;
mod srv;

pub use action::{
    GeneratedActionPackage, GeneratedCActionPackage, GeneratedNrosActionPackage,
    generate_action_package, generate_c_action_package, generate_nros_action_package,
    generate_nros_inline_action,
};
pub use common::GeneratorError;
pub use msg::{
    GeneratedCPackage, GeneratedNrosPackage, GeneratedPackage, generate_c_message_package,
    generate_message_package, generate_nros_inline_message, generate_nros_message_package,
};
pub use srv::{
    GeneratedCServicePackage, GeneratedNrosServicePackage, GeneratedServicePackage,
    generate_c_service_package, generate_nros_inline_service, generate_nros_service_package,
    generate_service_package,
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::RosEdition;
    use rosidl_parser::{
        Field, FieldType, PrimitiveType, parse_action, parse_message, parse_service,
    };
    use std::collections::HashSet;

    #[test]
    fn test_simple_message_generation() {
        let msg = parse_message("int32 x\nfloat64 y\n").unwrap();
        let deps = HashSet::new();

        let result = generate_message_package("test_msgs", "Point", &msg, &deps);
        assert!(result.is_ok());

        let pkg = result.unwrap();
        assert!(pkg.cargo_toml.contains("test_msgs"));
        assert!(pkg.message_rmw.contains("i32"));
        assert!(pkg.message_rmw.contains("f64"));
    }

    #[test]
    fn test_message_with_dependencies() {
        let msg = parse_message("geometry_msgs/Point position\n").unwrap();
        let deps = HashSet::new();

        let result = generate_message_package("nav_msgs", "Odometry", &msg, &deps);
        assert!(result.is_ok());

        let pkg = result.unwrap();
        assert!(pkg.cargo_toml.contains("geometry_msgs"));
    }

    #[test]
    fn test_message_with_large_array() {
        let mut msg = rosidl_parser::Message::new();
        msg.fields.push(Field {
            field_type: FieldType::Array {
                element_type: Box::new(FieldType::Primitive(PrimitiveType::Int32)),
                size: 64,
            },
            name: "data".to_string(),
            default_value: None,
        });

        let deps = HashSet::new();
        let result = generate_message_package("test_msgs", "LargeArray", &msg, &deps);
        assert!(result.is_ok());

        let pkg = result.unwrap();
        assert!(pkg.cargo_toml.contains("big-array"));
    }

    #[test]
    fn test_message_with_keyword_field() {
        let msg = parse_message("int32 type\nfloat64 match\n").unwrap();
        let deps = HashSet::new();

        let result = generate_message_package("test_msgs", "Keywords", &msg, &deps);
        assert!(result.is_ok());

        let pkg = result.unwrap();
        assert!(pkg.message_rmw.contains("type_"));
        assert!(pkg.message_rmw.contains("match_"));
    }

    #[test]
    fn test_simple_service_generation() {
        let srv = parse_service("int32 a\nint32 b\n---\nint32 sum\n").unwrap();
        let deps = HashSet::new();

        let result = generate_service_package("example_interfaces", "AddTwoInts", &srv, &deps);
        assert!(result.is_ok());

        let pkg = result.unwrap();
        assert!(pkg.cargo_toml.contains("example_interfaces"));
        assert!(pkg.lib_rs.contains("pub mod srv"));
        assert!(pkg.service_rmw.contains("AddTwoIntsRequest"));
        assert!(pkg.service_rmw.contains("AddTwoIntsResponse"));
        assert!(pkg.service_idiomatic.contains("AddTwoIntsRequest"));
        assert!(pkg.service_idiomatic.contains("AddTwoIntsResponse"));
    }

    #[test]
    fn test_service_with_dependencies() {
        let srv = parse_service("geometry_msgs/Point position\n---\nbool success\n").unwrap();
        let deps = HashSet::new();

        let result = generate_service_package("test_srvs", "CheckPoint", &srv, &deps);
        assert!(result.is_ok());

        let pkg = result.unwrap();
        assert!(pkg.cargo_toml.contains("geometry_msgs"));
    }

    #[test]
    fn test_simple_action_generation() {
        let action =
            parse_action("int32 order\n---\nint32[] sequence\n---\nint32[] partial_sequence\n")
                .unwrap();
        let deps = HashSet::new();

        let result = generate_action_package("example_interfaces", "Fibonacci", &action, &deps);
        assert!(result.is_ok());

        let pkg = result.unwrap();
        assert!(pkg.cargo_toml.contains("example_interfaces"));
        assert!(pkg.lib_rs.contains("pub mod action"));
        assert!(pkg.action_rmw.contains("FibonacciGoal"));
        assert!(pkg.action_rmw.contains("FibonacciResult"));
        assert!(pkg.action_rmw.contains("FibonacciFeedback"));
        assert!(pkg.action_idiomatic.contains("FibonacciGoal"));
        assert!(pkg.action_idiomatic.contains("FibonacciResult"));
        assert!(pkg.action_idiomatic.contains("FibonacciFeedback"));
    }

    #[test]
    fn test_action_with_dependencies() {
        let action = parse_action(
            "geometry_msgs/Point target\n---\nfloat64 distance\n---\nfloat64 current_distance\n",
        )
        .unwrap();
        let deps = HashSet::new();

        let result = generate_action_package("test_actions", "Navigate", &action, &deps);
        assert!(result.is_ok());

        let pkg = result.unwrap();
        assert!(pkg.cargo_toml.contains("geometry_msgs"));
    }

    // ========================================================================
    // nros Backend Tests
    // ========================================================================

    #[test]
    fn test_nros_simple_message_generation() {
        let msg = parse_message("int32 x\nfloat64 y\nstring name\n").unwrap();
        let deps = HashSet::new();

        let result = generate_nros_message_package(
            "test_msgs",
            "Point",
            &msg,
            &deps,
            "0.1.0",
            RosEdition::Humble,
        );
        assert!(result.is_ok());

        let pkg = result.unwrap();

        // Check Cargo.toml has nros dependencies
        assert!(pkg.cargo_toml.contains("nros-core"));
        assert!(pkg.cargo_toml.contains("nros-serdes"));
        assert!(pkg.cargo_toml.contains("heapless"));

        // Check lib.rs is no_std
        assert!(pkg.lib_rs.contains("#![no_std]"));
        assert!(pkg.lib_rs.contains("pub mod msg"));

        // Check message contains proper types
        assert!(pkg.message_rs.contains("pub x: i32"));
        assert!(pkg.message_rs.contains("pub y: f64"));
        assert!(pkg.message_rs.contains("heapless::String<256>"));

        // Check it has Serialize/Deserialize implementations
        assert!(pkg.message_rs.contains("impl Serialize for Point"));
        assert!(pkg.message_rs.contains("impl Deserialize for Point"));
        assert!(pkg.message_rs.contains("impl RosMessage for Point"));
    }

    #[test]
    fn test_nros_message_with_sequence() {
        let msg = parse_message("int32[] data\n").unwrap();
        let deps = HashSet::new();

        let result = generate_nros_message_package(
            "test_msgs",
            "IntArray",
            &msg,
            &deps,
            "0.1.0",
            RosEdition::Humble,
        );
        assert!(result.is_ok());

        let pkg = result.unwrap();
        // Check sequence uses heapless::Vec
        assert!(pkg.message_rs.contains("heapless::Vec<i32"));
    }

    #[test]
    fn test_nros_service_generation() {
        let srv = parse_service("int64 a\nint64 b\n---\nint64 sum\n").unwrap();
        let deps = HashSet::new();

        let result = generate_nros_service_package(
            "test_srvs",
            "AddTwoInts",
            &srv,
            &deps,
            "0.1.0",
            RosEdition::Humble,
        );
        assert!(result.is_ok());

        let pkg = result.unwrap();

        // Check Cargo.toml
        assert!(pkg.cargo_toml.contains("nros-core"));

        // Check lib.rs
        assert!(pkg.lib_rs.contains("pub mod srv"));

        // Check service types
        assert!(pkg.service_rs.contains("AddTwoIntsRequest"));
        assert!(pkg.service_rs.contains("AddTwoIntsResponse"));
        assert!(pkg.service_rs.contains("pub a: i64"));
        assert!(pkg.service_rs.contains("pub b: i64"));
        assert!(pkg.service_rs.contains("pub sum: i64"));

        // Check RosService impl
        assert!(pkg.service_rs.contains("impl RosService for AddTwoInts"));
    }

    #[test]
    fn test_nros_action_generation() {
        let action =
            parse_action("int32 order\n---\nint32[] sequence\n---\nint32[] partial_sequence\n")
                .unwrap();
        let deps = HashSet::new();

        let result = generate_nros_action_package(
            "example_interfaces",
            "Fibonacci",
            &action,
            &deps,
            "0.1.0",
            RosEdition::Humble,
        );
        assert!(result.is_ok());

        let pkg = result.unwrap();

        // Check Cargo.toml
        assert!(pkg.cargo_toml.contains("nros-core"));

        // Check lib.rs
        assert!(pkg.lib_rs.contains("pub mod action"));

        // Check action types
        assert!(pkg.action_rs.contains("FibonacciGoal"));
        assert!(pkg.action_rs.contains("FibonacciResult"));
        assert!(pkg.action_rs.contains("FibonacciFeedback"));
        assert!(pkg.action_rs.contains("pub order: i32"));

        // Check RosAction impl
        assert!(pkg.action_rs.contains("impl RosAction for Fibonacci"));
        assert!(pkg.action_rs.contains("type Goal = FibonacciGoal"));
        assert!(pkg.action_rs.contains("type Result = FibonacciResult"));
        assert!(pkg.action_rs.contains("type Feedback = FibonacciFeedback"));
    }

    // ========================================================================
    // C Code Generation Tests
    // ========================================================================

    #[test]
    fn test_c_simple_message_generation() {
        let msg = parse_message("int32 x\nfloat64 y\nbool flag\n").unwrap();
        let type_hash = "abc123";

        let result = generate_c_message_package("test_msgs", "Point", &msg, type_hash);
        assert!(result.is_ok());

        let pkg = result.unwrap();

        // Check header file
        assert!(pkg.header.contains("#ifndef TEST_MSGS_MSG_POINT_H"));
        assert!(pkg.header.contains("typedef struct test_msgs_msg_point"));
        assert!(pkg.header.contains("int32_t x"));
        assert!(pkg.header.contains("double y"));
        assert!(pkg.header.contains("bool flag"));
        assert!(pkg.header.contains("test_msgs_msg_point_init"));
        assert!(pkg.header.contains("test_msgs_msg_point_serialize"));
        assert!(pkg.header.contains("test_msgs_msg_point_deserialize"));

        // Check source file
        assert!(pkg.source.contains("test_msgs_msg_point.h"));
        assert!(pkg.source.contains("nros_cdr_write_i32"));
        assert!(pkg.source.contains("nros_cdr_write_f64"));
        assert!(pkg.source.contains("nros_cdr_write_bool"));

        // Check file names
        assert_eq!(pkg.header_name, "test_msgs_msg_point.h");
        assert_eq!(pkg.source_name, "test_msgs_msg_point.c");
    }

    #[test]
    fn test_c_message_with_string() {
        let msg = parse_message("string name\n").unwrap();
        let type_hash = "def456";

        let result = generate_c_message_package("std_msgs", "String", &msg, type_hash);
        assert!(result.is_ok());

        let pkg = result.unwrap();
        assert!(pkg.header.contains("char name[256]"));
        assert!(pkg.source.contains("nros_cdr_write_string"));
    }

    #[test]
    fn test_c_message_with_array() {
        let msg = parse_message("int32[3] values\n").unwrap();
        let type_hash = "ghi789";

        let result = generate_c_message_package("test_msgs", "IntArray", &msg, type_hash);
        assert!(result.is_ok());

        let pkg = result.unwrap();
        assert!(pkg.header.contains("int32_t values[3]"));
        assert!(pkg.source.contains("for (size_t i = 0; i < 3; ++i)"));
    }

    #[test]
    fn test_c_simple_service_generation() {
        let srv = parse_service("int32 a\nint32 b\n---\nint32 sum\n").unwrap();
        let type_hash = "srv123";

        let result = generate_c_service_package("test_srvs", "AddTwoInts", &srv, type_hash);
        assert!(result.is_ok());

        let pkg = result.unwrap();

        // Check header file
        assert!(pkg.header.contains("#ifndef TEST_SRVS_SRV_ADD_TWO_INTS_H"));
        assert!(
            pkg.header
                .contains("typedef struct test_srvs_srv_add_two_ints_request")
        );
        assert!(
            pkg.header
                .contains("typedef struct test_srvs_srv_add_two_ints_response")
        );
        assert!(pkg.header.contains("int32_t a"));
        assert!(pkg.header.contains("int32_t b"));
        assert!(pkg.header.contains("int32_t sum"));

        // Check source file
        assert!(
            pkg.source
                .contains("test_srvs_srv_add_two_ints_request_init")
        );
        assert!(
            pkg.source
                .contains("test_srvs_srv_add_two_ints_response_init")
        );
        assert!(
            pkg.source
                .contains("test_srvs_srv_add_two_ints_request_serialize")
        );
        assert!(
            pkg.source
                .contains("test_srvs_srv_add_two_ints_response_serialize")
        );

        // Check file names
        assert_eq!(pkg.header_name, "test_srvs_srv_add_two_ints.h");
        assert_eq!(pkg.source_name, "test_srvs_srv_add_two_ints.c");
    }

    #[test]
    fn test_c_simple_action_generation() {
        let action =
            parse_action("int32 order\n---\nint32 result_code\n---\nint32 progress\n").unwrap();
        let type_hash = "act456";

        let result = generate_c_action_package("test_actions", "Fibonacci", &action, type_hash);
        assert!(result.is_ok());

        let pkg = result.unwrap();

        // Check header file
        assert!(
            pkg.header
                .contains("#ifndef TEST_ACTIONS_ACTION_FIBONACCI_H")
        );
        assert!(
            pkg.header
                .contains("typedef struct test_actions_action_fibonacci_goal")
        );
        assert!(
            pkg.header
                .contains("typedef struct test_actions_action_fibonacci_result")
        );
        assert!(
            pkg.header
                .contains("typedef struct test_actions_action_fibonacci_feedback")
        );
        assert!(pkg.header.contains("int32_t order"));
        assert!(pkg.header.contains("int32_t result_code"));
        assert!(pkg.header.contains("int32_t progress"));

        // Check source file
        assert!(
            pkg.source
                .contains("test_actions_action_fibonacci_goal_init")
        );
        assert!(
            pkg.source
                .contains("test_actions_action_fibonacci_result_init")
        );
        assert!(
            pkg.source
                .contains("test_actions_action_fibonacci_feedback_init")
        );

        // Check file names
        assert_eq!(pkg.header_name, "test_actions_action_fibonacci.h");
        assert_eq!(pkg.source_name, "test_actions_action_fibonacci.c");
    }
}

use super::helpers::fixtures::get_language;
use tree_sitter_tags::{TagKind, TagsConfiguration, TagsContext};

#[test]
fn test_tags_javascript() {
    let language = get_language("python");
    let tags_config = TagsConfiguration::new(
        language,
        r#"
        ((function_definition
            name: (identifier) @name
            body: (block
                . (string) @doc)) @function
         (set! replace @doc "(^['\s]*)|(['\s]*$)"))

        (function_definition
            name: (identifier) @name) @function
        (class_definition
            name: (identifier) @name) @class
        (call
            function: (identifier) @name) @call
        "#,
        "",
    )
    .unwrap();

    let mut tag_context = TagsContext::new();
    let tags = tag_context.generate_tags(
        &tags_config,
        br#"
        class Customer:
            """
            Data about a customer
            """

            def age(self):
                """
                Get the customer's age
                """
                compute_age(self.id);
        }
        "#,
    );

    assert_eq!(
        tags.iter().map(|t| (t.name, t.kind)).collect::<Vec<_>>(),
        &[
            ("Customer", TagKind::Class),
            ("age", TagKind::Function),
            ("compute_age", TagKind::Call),
        ]
    );

    assert_eq!(tags[0].docs, Some("Data about a customer"));
    assert_eq!(tags[1].docs, Some("Get the customer's age"));
}
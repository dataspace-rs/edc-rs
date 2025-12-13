#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::types::policy::{AtomicConstraint, Constraint, Operator, Policy, PolicyKind};

    #[test]
    fn should_deserialize_odrl() {
        let json = json!({
            "@type": "Set",
            "assigner": "assigner",
            "assignee": "assignee",
            "target": "target",
            "obligation": [{
                "action": "display",
                "constraint": [{
                   "leftOperand": "spatial",
                   "operator": "eq",
                   "rightOperand":  "https://www.wikidata.org/resource/Q183",
               }]
            }],
            "permission": [{
                "action": "display",
                "constraint": [{
                   "leftOperand": "spatial",
                   "operator": "eq",
                   "rightOperand":  "https://www.wikidata.org/resource/Q183",
               }]
            }],
            "prohibition": [{
                "action": "display",
                "constraint": [{
                   "leftOperand": "spatial",
                   "operator": "eq",
                   "rightOperand":  "https://www.wikidata.org/resource/Q183",
               }]
            }]
        });

        let policy = serde_json::from_value::<Policy>(json.clone()).unwrap();

        let serialized = serde_json::to_value(&policy).unwrap();

        assert_eq!(&json, &serialized);
    }

    #[test]
    fn should_deserialize_odrl_with_multiplicity_constraints() {
        let json = json!({
            "@type":"Set",
            "assigner":"assigner",
            "assignee":"assignee",
            "target":"target",
            "obligation":[
                {
                    "action":"display",
                    "constraint":[{
                        "and":[
                            {
                                "leftOperand":"spatial",
                                "operator":"eq",
                                "rightOperand":"https://www.wikidata.org/resource/Q183"
                            }
                        ]
                    }]
                }
            ],
        });

        let policy = serde_json::from_value::<Policy>(json.clone()).unwrap();

        let serialized = serde_json::to_value(&policy).unwrap();

        assert_eq!(&json, &serialized);
    }

    #[test]
    fn should_deserialize_edc_prefixed() {
        let json = json!({
            "@id": "b3e9255b-14c9-4a2b-a439-50b9382b81b1",
            "@type": "odrl:Set",
            "odrl:permission": {
                "odrl:action": {
                    "@id": "http://www.w3.org/ns/odrl/2/use"
                },
                "odrl:constraint": {
                    "odrl:leftOperand": {
                      "@id": "https://w3id.org/edc/v0.0.1/ns/foo"
                    },
                    "odrl:operator": {
                        "@id": "odrl:eq"
                    },
                    "odrl:rightOperand": "bar"
                }
            },
            "odrl:prohibition": [],
            "odrl:obligation": []
        });

        let policy = serde_json::from_value::<Policy>(json).unwrap();

        assert_eq!(policy.kind(), &PolicyKind::Set);
        assert_eq!(policy.permissions().len(), 1);

        let permission = &policy.permissions[0];

        assert_eq!(permission.action().id(), "http://www.w3.org/ns/odrl/2/use");
        assert_eq!(permission.constraints().len(), 1);

        let constraint = &permission.constraints()[0];

        assert_eq!(
            constraint,
            &Constraint::Atomic(AtomicConstraint::new_with_operator(
                "https://w3id.org/edc/v0.0.1/ns/foo",
                Operator::id("odrl:eq"),
                "bar"
            ))
        );
    }
}

//! Results of the policy definition `validate` and `evaluationplan` endpoints.

use bon::Builder;
use serde::{Deserialize, Serialize};
use serde_with::{formats::PreferMany, serde_as, OneOrMany};

/// Outcome of `POST /policydefinitions/{id}/validate`.
#[serde_as]
#[derive(Debug, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PolicyValidationResult {
    #[serde(default)]
    is_valid: bool,
    #[serde(default)]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    errors: Vec<String>,
}

impl PolicyValidationResult {
    pub fn is_valid(&self) -> bool {
        self.is_valid
    }

    pub fn errors(&self) -> &[String] {
        &self.errors
    }
}

/// Request body of `POST /policydefinitions/{id}/evaluationplan`.
#[derive(Debug, Serialize, Clone, Builder)]
#[serde(rename_all = "camelCase")]
pub struct PolicyEvaluationPlanRequest {
    #[builder(default = "PolicyEvaluationPlanRequest".to_string())]
    #[serde(rename = "@type")]
    ty: String,
    /// The policy scope to build the plan for, e.g. `catalog` or
    /// `contract.negotiation`.
    #[builder(into)]
    policy_scope: String,
}

impl PolicyEvaluationPlanRequest {
    pub fn policy_scope(&self) -> &str {
        &self.policy_scope
    }
}

/// How the policy engine would evaluate a policy in a given scope: which
/// validators run and which rules/constraints are evaluated or filtered out.
#[serde_as]
#[derive(Debug, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PolicyEvaluationPlan {
    #[serde(default)]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    pre_validators: Vec<String>,
    #[serde(default)]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    post_validators: Vec<String>,
    #[serde(default)]
    permission_steps: Vec<RuleStep>,
    #[serde(default)]
    prohibition_steps: Vec<RuleStep>,
    /// Steps for the policy obligations. The connector emits them as
    /// `obligationSteps`; `dutySteps` is accepted as an alias.
    #[serde(default, alias = "dutySteps")]
    obligation_steps: Vec<RuleStep>,
}

impl PolicyEvaluationPlan {
    pub fn pre_validators(&self) -> &[String] {
        &self.pre_validators
    }

    pub fn post_validators(&self) -> &[String] {
        &self.post_validators
    }

    pub fn permission_steps(&self) -> &[RuleStep] {
        &self.permission_steps
    }

    pub fn prohibition_steps(&self) -> &[RuleStep] {
        &self.prohibition_steps
    }

    pub fn obligation_steps(&self) -> &[RuleStep] {
        &self.obligation_steps
    }
}

/// Evaluation step of a single rule (`PermissionStep`, `ProhibitionStep` or
/// `DutyStep`, see [`ty`](RuleStep::ty)).
///
/// Lists of steps are plain `Vec`s on purpose: every field of a step has a
/// default, so an empty JSON array would otherwise deserialize as one empty
/// step through `OneOrMany`.
#[serde_as]
#[derive(Debug, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct RuleStep {
    #[serde(rename = "@type", default)]
    ty: String,
    #[serde(default)]
    is_filtered: bool,
    #[serde(default)]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    filtering_reasons: Vec<String>,
    /// Names of the rule functions bound to this rule's action.
    #[serde(default)]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    rule_functions: Vec<String>,
    #[serde(default)]
    constraint_steps: Vec<ConstraintStep>,
    /// Only populated for permission steps.
    #[serde(default)]
    duty_steps: Vec<RuleStep>,
}

impl RuleStep {
    pub fn ty(&self) -> &str {
        &self.ty
    }

    pub fn is_filtered(&self) -> bool {
        self.is_filtered
    }

    pub fn filtering_reasons(&self) -> &[String] {
        &self.filtering_reasons
    }

    pub fn rule_functions(&self) -> &[String] {
        &self.rule_functions
    }

    pub fn constraint_steps(&self) -> &[ConstraintStep] {
        &self.constraint_steps
    }

    pub fn duty_steps(&self) -> &[RuleStep] {
        &self.duty_steps
    }
}

/// Evaluation step of a constraint, discriminated by its `@type`.
#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "@type")]
pub enum ConstraintStep {
    AtomicConstraintStep(AtomicConstraintStep),
    AndConstraintStep(MultiplicityConstraintStep),
    OrConstraintStep(MultiplicityConstraintStep),
    XoneConstraintStep(MultiplicityConstraintStep),
    /// A step kind unknown to this client; its payload is not retained.
    #[serde(other)]
    Other,
}

#[serde_as]
#[derive(Debug, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct AtomicConstraintStep {
    #[serde(default)]
    is_filtered: bool,
    #[serde(default)]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    filtering_reasons: Vec<String>,
    #[serde(default)]
    function_name: Option<String>,
    #[serde(default)]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    function_params: Vec<String>,
}

impl AtomicConstraintStep {
    pub fn is_filtered(&self) -> bool {
        self.is_filtered
    }

    pub fn filtering_reasons(&self) -> &[String] {
        &self.filtering_reasons
    }

    pub fn function_name(&self) -> Option<&str> {
        self.function_name.as_deref()
    }

    pub fn function_params(&self) -> &[String] {
        &self.function_params
    }
}

/// `and`, `or` and `xone` steps: a list of nested constraint steps.
#[serde_as]
#[derive(Debug, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct MultiplicityConstraintStep {
    #[serde(default)]
    constraint_steps: Vec<ConstraintStep>,
}

impl MultiplicityConstraintStep {
    pub fn constraint_steps(&self) -> &[ConstraintStep] {
        &self.constraint_steps
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use serde_json::json;

    use super::{ConstraintStep, PolicyEvaluationPlan, PolicyEvaluationPlanRequest};

    #[test]
    fn should_serialize_a_plan_request() {
        let request = PolicyEvaluationPlanRequest::builder()
            .policy_scope("catalog")
            .build();

        assert_eq!(
            serde_json::to_value(&request).unwrap(),
            json!({"@type": "PolicyEvaluationPlanRequest", "policyScope": "catalog"})
        );
    }

    #[test]
    fn should_deserialize_a_nested_plan() {
        let plan: PolicyEvaluationPlan = serde_json::from_value(json!({
            "@type": "PolicyEvaluationPlan",
            "preValidators": ["pre"],
            "postValidators": [],
            "permissionSteps": [{
                "@type": "PermissionStep",
                "isFiltered": false,
                "filteringReasons": [],
                "ruleFunctions": ["fn"],
                "constraintSteps": [{
                    "@type": "AndConstraintStep",
                    "constraintSteps": [
                        {
                            "@type": "AtomicConstraintStep",
                            "isFiltered": true,
                            "filteringReasons": ["a", "b"],
                            "functionParams": ["left", "EQ", "right"]
                        },
                        {"@type": "FutureStep", "whatever": 1}
                    ]
                }],
                "dutySteps": [{"@type": "DutyStep", "isFiltered": false}]
            }],
            "prohibitionSteps": [],
            "obligationSteps": [{"@type": "DutyStep"}]
        }))
        .unwrap();

        assert_eq!(plan.pre_validators(), ["pre"]);
        let permission = &plan.permission_steps()[0];
        assert_eq!(permission.ty(), "PermissionStep");
        assert_eq!(permission.rule_functions(), ["fn"]);
        assert_eq!(permission.duty_steps().len(), 1);
        let ConstraintStep::AndConstraintStep(and) = &permission.constraint_steps()[0] else {
            panic!("expected an and step");
        };
        let ConstraintStep::AtomicConstraintStep(atomic) = &and.constraint_steps()[0] else {
            panic!("expected an atomic step");
        };
        assert!(atomic.is_filtered());
        assert_eq!(atomic.function_name(), None);
        assert_eq!(atomic.function_params().len(), 3);
        assert!(matches!(and.constraint_steps()[1], ConstraintStep::Other));
        assert_eq!(plan.obligation_steps().len(), 1);
    }

    #[test]
    fn should_deserialize_the_connector_plan_for_a_plain_permission() {
        let plan: PolicyEvaluationPlan = serde_json::from_value(json!({
            "@type": "PolicyEvaluationPlan",
            "preValidators": [],
            "permissionSteps": [{
                "@type": "PermissionStep", "isFiltered": false, "filteringReasons": [],
                "ruleFunctions": [], "constraintSteps": [], "dutySteps": []
            }],
            "prohibitionSteps": [],
            "obligationSteps": [],
            "postValidators": [],
            "@context": ["https://w3id.org/edc/connector/management/v2"]
        }))
        .unwrap();

        assert_eq!(1, plan.permission_steps().len(), "{plan:?}");
        assert!(plan.prohibition_steps().is_empty(), "{plan:?}");
        assert!(plan.obligation_steps().is_empty(), "{plan:?}");
        assert!(plan.pre_validators().is_empty(), "{plan:?}");
    }

    #[test]
    fn should_accept_duty_steps_alias_and_missing_fields() {
        let plan: PolicyEvaluationPlan =
            serde_json::from_value(json!({"dutySteps": [{"@type": "DutyStep"}]})).unwrap();

        assert_eq!(plan.obligation_steps().len(), 1);
        assert!(plan.permission_steps().is_empty());
    }
}

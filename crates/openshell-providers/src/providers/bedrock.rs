// SPDX-FileCopyrightText: Copyright (c) 2025-2026 NVIDIA CORPORATION & AFFILIATES. All rights reserved.
// SPDX-License-Identifier: Apache-2.0

use crate::{
    ProviderDiscoverySpec, ProviderError, ProviderPlugin, RealDiscoveryContext, discover_with_spec,
};

pub struct BedrockProvider;

pub const SPEC: ProviderDiscoverySpec = ProviderDiscoverySpec {
    id: "bedrock",
    credential_env_vars: &[
        "AWS_ACCESS_KEY_ID",
        "AWS_SECRET_ACCESS_KEY",
        "AWS_SESSION_TOKEN",
    ],
};

impl ProviderPlugin for BedrockProvider {
    fn id(&self) -> &'static str {
        SPEC.id
    }

    fn discover_existing(&self) -> Result<Option<crate::DiscoveredProvider>, ProviderError> {
        discover_with_spec(&SPEC, &RealDiscoveryContext)
    }

    fn credential_env_vars(&self) -> &'static [&'static str] {
        SPEC.credential_env_vars
    }
}

#[cfg(test)]
mod tests {
    use super::SPEC;
    use crate::discover_with_spec;
    use crate::test_helpers::MockDiscoveryContext;

    #[test]
    fn discovers_bedrock_env_credentials() {
        let ctx = MockDiscoveryContext::new()
            .with_env("AWS_ACCESS_KEY_ID", "AKIAIOSFODNN7EXAMPLE")
            .with_env("AWS_SECRET_ACCESS_KEY", "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY");
        let discovered = discover_with_spec(&SPEC, &ctx)
            .expect("discovery")
            .expect("provider");
        assert_eq!(
            discovered.credentials.get("AWS_ACCESS_KEY_ID"),
            Some(&"AKIAIOSFODNN7EXAMPLE".to_string())
        );
        assert_eq!(
            discovered.credentials.get("AWS_SECRET_ACCESS_KEY"),
            Some(&"wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY".to_string())
        );
    }

    #[test]
    fn discovers_bedrock_session_token() {
        let ctx = MockDiscoveryContext::new()
            .with_env("AWS_ACCESS_KEY_ID", "AKIAIOSFODNN7EXAMPLE")
            .with_env("AWS_SECRET_ACCESS_KEY", "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY")
            .with_env("AWS_SESSION_TOKEN", "FwoGZXIvYXdzEBY-session-token");
        let discovered = discover_with_spec(&SPEC, &ctx)
            .expect("discovery")
            .expect("provider");
        assert_eq!(
            discovered.credentials.get("AWS_SESSION_TOKEN"),
            Some(&"FwoGZXIvYXdzEBY-session-token".to_string())
        );
    }

    #[test]
    fn returns_none_without_aws_credentials() {
        let ctx = MockDiscoveryContext::new();
        let discovered = discover_with_spec(&SPEC, &ctx).expect("discovery");
        assert!(discovered.is_none());
    }
}
anchor_gen::generate_cpi_crate!("idl.json");

anchor_lang::declare_id!("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4");

use solana_program::pubkey;

// Temporarily redefined it until solution is found
pub mod jupiter_override {
    use anchor_lang::InstructionData;
    use anchor_lang::{prelude::*, Discriminator};
    use jupiter_amm_interface::Swap as InterfaceSwap;

    #[derive(AnchorSerialize, Debug)]
    pub struct RoutePlanStep {
        pub swap: InterfaceSwap,
        pub percent: u8,
        pub input_index: u8,
        pub output_index: u8,
    }

    #[derive(AnchorSerialize)]
    pub struct Route {
        pub route_plan: Vec<RoutePlanStep>,
        pub in_amount: u64,
        pub quoted_out_amount: u64,
        pub slippage_bps: u16,
        pub platform_fee_bps: u8,
    }
    impl Discriminator for Route {
        const DISCRIMINATOR: [u8; 8] = super::instruction::Route::DISCRIMINATOR;
    }

    impl InstructionData for Route {}

    #[derive(AnchorSerialize)]
    pub struct RouteWithTokenLedger {
        pub route_plan: Vec<RoutePlanStep>,
        pub quoted_out_amount: u64,
        pub slippage_bps: u16,
        pub platform_fee_bps: u8,
    }
    impl Discriminator for RouteWithTokenLedger {
        const DISCRIMINATOR: [u8; 8] = super::instruction::RouteWithTokenLedger::DISCRIMINATOR;
    }

    impl InstructionData for RouteWithTokenLedger {}

    #[derive(AnchorSerialize)]
    pub struct SharedAccountsRoute {
        pub id: u8,
        pub route_plan: Vec<RoutePlanStep>,
        pub in_amount: u64,
        pub quoted_out_amount: u64,
        pub slippage_bps: u16,
        pub platform_fee_bps: u8,
    }
    impl Discriminator for SharedAccountsRoute {
        const DISCRIMINATOR: [u8; 8] = super::instruction::SharedAccountsRoute::DISCRIMINATOR;
    }

    impl InstructionData for SharedAccountsRoute {}

    #[derive(AnchorSerialize)]
    pub struct SharedAccountsRouteWithTokenLedger {
        pub id: u8,
        pub route_plan: Vec<RoutePlanStep>,
        pub quoted_out_amount: u64,
        pub slippage_bps: u16,
        pub platform_fee_bps: u8,
    }
    impl Discriminator for SharedAccountsRouteWithTokenLedger {
        const DISCRIMINATOR: [u8; 8] =
            super::instruction::SharedAccountsRouteWithTokenLedger::DISCRIMINATOR;
    }

    impl InstructionData for SharedAccountsRouteWithTokenLedger {}
}

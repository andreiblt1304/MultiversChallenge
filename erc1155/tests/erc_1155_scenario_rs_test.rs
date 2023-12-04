use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract("file:output/erc_1155.wasm", erc1155::ContractBuilder);
    blockchain
}

#[test]
fn empty_rs() {
    world().run("scenarios/erc_1155.scen.json");
}

#[test]
fn deploy() {
    world().run("scenarios/erc_1155-deploy.scen.json");
}

// create
#[test]
fn create_fungible() {
    world().run("scenarios/erc_1155-create-fungible.scen.json");
}

#[test]
fn create_nonfungible() {
    world().run("scenarios/erc_1155-create-nonfungible.scen.json");
}

#[test]
fn create_two_fungible_same_creator() {
    world().run("scenarios/erc_1155-create-two-fungible-same-creator.scen.json");
}

#[test]
fn create_two_fungible_different_creator() {
    world().run("scenarios/erc_1155-create-two-fungible-different-creator.scen.json");
}

// mint
#[test]
fn mint_fungible() {
    world().run("scenarios/erc_1155-mint-fungible.scen.json");
}

#[test]
fn mint_nonfungible() {
    world().run("scenarios/erc_1155-mint-nonfungible.scen.json");
}

// burn
#[test]
fn burn_fungible() {
    world().run("scenarios/erc_1155-burn-fungible.scen.json");
}

#[test]
fn burn_nonfungible() {
    world().run("scenarios/erc_1155-burn-nonfungible.scen.json");
}

// transfer
#[test]
fn transfer_fungible() {
    world().run("scenarios/erc_1155-safe-transfer-fungible-ok.scen.json");
}

#[test]
fn transfer_nonfungible() {
    world().run("scenarios/erc_1155-safe-transfer-nonfungible-ok.scen.json");
}

#[test]
fn batch_tranfer_two_fungible() {
    world().run("scenarios/erc_1155-batch-transfer-two-fungible.scen.json");
}

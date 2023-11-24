use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    // blockchain.set_current_dir_from_workspace("relative path to your workspace, if applicable");

    blockchain.register_contract("file:output/erc1155.wasm", erc_1155::ContractBuilder);
    blockchain
}

#[test]
fn empty_rs() {
    world().run("scenarios/erc_1155.scen.json");
}

use substreams::errors::Error;
use substreams::log;
use substreams_antelope::pb::Block;

use crate::abi;
use crate::setabi::*;

#[substreams::handlers::map]
fn map_setabis(block: Block) -> Result<SetABIEvents, Error> {
    let mut setabis = vec![];

    for trx in block.all_transaction_traces() {
        // action traces
        for trace in &trx.action_traces {
            let action_trace = trace.action.as_ref().unwrap();
            // parse setabi actions
            if action_trace.account == "eosio" && action_trace.name == "setabi" {
                match abi::Setabi::try_from(action_trace.json_data.as_str()) {
                    Ok(data) => {
                        let contract = action_trace.account.clone();

                        setabis.push(SetABIEvent {
                            // trace information
                            trx_id: trx.id.clone(),
                            action_ordinal: trace.action_ordinal,

                            // contract & scope
                            contract,
                            action: action_trace.name.clone(),

                            // payload
                            account: data.account,
                            abi: data.abi,
                        });
                    }
                    Err(_) => continue,
                }
            }
        }
    }
    Ok(SetABIEvents { items: setabis })
}

//
// Copyright (c) 2017, 2020 ADLINK Technology Inc.
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ADLINK zenoh team, <zenoh@adlink-labs.tech>
//
use async_std::prelude::*;
use zenoh::net::*;

#[async_std::main]
async fn main() {
    // Open a Zenoh session using the default configuration
    let session = open(Config::default(), None).await.unwrap();
    // Declare a queryable resource to react on
    let (res, val) = ("/region01/house01/temp", "25");
    let mut queryable = session
        .declare_queryable(&res.into(), queryable::EVAL)
        .await
        .unwrap();
    // Process the incoming queries
    while let Some(query) = queryable.stream().next().await {
        println!(
            ">> Replying to '{}': ('{}': '{}')",
            query.res_name, res, val
        );
        // Generate and send back the response
        let sample = Sample {
            res_name: res.to_string(),
            payload: val.as_bytes().into(),
            data_info: None,
        };
        query.reply(sample).await;
    }
}

nohup ./target/release/node-template --base-path /data/substrate/node03 --chain ./customSpecRaw.json --port 30336  --ws-port 9946  --rpc-port 9936   --validator  --rpc-methods Unsafe  --name sfc03 --bootnodes /ip4/172.16.0.13/tcp/30335/p2p/12D3KooWPr5x7aiKdDVMSEiXRvboJpznYPRX62nkFVy9abqiQzLE --rpc-cors all --ws-external  > logs/sfc03.log 2>&1 &
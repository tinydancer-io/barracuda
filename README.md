# Barracuda
A parallel-network of solana validators serving light clients on Solana.

It comprises of the on-chain contracts, the plugin and the p2p network.

The initial goal is that the stake can have consensus over transaction receipts in a block that light clients can verify with minimal effort. The eventual goal is to expand it to a DA layer on solana that is immune to erasure fraud and supports DAS at a block level.


## Credits
We are using the [jito-geyser-protos](https://github.com/jito-foundation/geyser-grpc-plugin) included in the `protos` folder built by the jito-labs team, we are grateful to their work and thank them for allowing us to use it.


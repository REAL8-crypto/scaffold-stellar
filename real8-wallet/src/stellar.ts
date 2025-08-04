import {
  Asset,
  TransactionBuilder,
  Operation,
  Horizon,
  Transaction,
} from "@stellar/stellar-sdk";
import { network, stellarNetwork } from "./contracts/util";

const server = new Horizon.Server(network.horizonUrl, {
  allowHttp: stellarNetwork === "LOCAL",
});

// REAL8 asset definition (adjust the code and issuer as needed)
// TODO: It's recommended to manage contract IDs and asset issuers in a centralized
// configuration file that can vary by network (e.g., local, testnet, mainnet).
const REAL8_ISSUER_MAINNET =
  "GBVYYQ7XXRZW6ZCNNCL2X2THNPQ6IM4O47HAA25JTAG7Z3CXJCQ3W4CD";
const REAL8_ISSUER_TESTNET =
  "GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF"; // Replace with your Testnet issuer address

const REAL8_ISSUER =
  network.id === "mainnet" ? REAL8_ISSUER_MAINNET : REAL8_ISSUER_TESTNET;

const REAL8 = new Asset("REAL8", REAL8_ISSUER);

/**
 * Builds a transaction to add a trustline for the REAL8 asset.
 * This transaction must be signed by the user's wallet and then submitted.
 * @param {string} accountId The public key of the account adding the trustline.
 * @returns {Promise<string>} A base64-encoded transaction XDR.
 */
export async function buildAddTrustlineTx(accountId: string): Promise<string> {
  const account = await server.loadAccount(accountId);
  const transaction = new TransactionBuilder(account, {
    fee: await server.fetchBaseFee(),
    networkPassphrase: network.passphrase,
  })
    .addOperation(Operation.changeTrust({ asset: REAL8 }))
    .setTimeout(30)
    .build();

  return transaction.toXDR();
}

/**
 * Submits a signed transaction to the Stellar network.
 * @param {string} signedTxXdr The base64-encoded signed transaction XDR.
 * @returns The submission response from Horizon.
 */
export async function submitTx(signedTxXdr: string) {
  const tx = TransactionBuilder.fromXDR(
    signedTxXdr,
    network.passphrase,
  ) as Transaction;
  return server.submitTransaction(tx);
}

export async function fetchLiquidityPools() {
  const pools = await server.liquidityPools().forAsset(REAL8).call();
  return pools.records;
}
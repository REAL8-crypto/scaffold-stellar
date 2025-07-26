import { Server, Asset, Operation, TransactionBuilder, Keypair } from '@stellar/stellar-sdk';

// Connect to the Stellar public network
export const server = new Server('https://horizon.stellar.org');

// REAL8 asset definition
export const REAL8 = new Asset(
  'REAL8',
  'GBVYYQ7XXRZW6ZCNNCL2X2THNPQ6IM4O47HAA25JTAG7Z3CXJCQ3W4CD'
);

/**
 * Fetches all balances for a given Stellar account.
 * @param accountId - Stellar public key
 */
export async function fetchBalances(accountId: string) {
  const account = await server.loadAccount(accountId);
  return account.balances;
}

/**
 * Gets the REAL8 balance for a Stellar account.
 * @param accountId - Stellar public key
 */
export async function fetchREAL8Balance(accountId: string) {
  const balances = await fetchBalances(accountId);
  const real8 = balances.find(
    (bal) =>
      bal.asset_code === 'REAL8' &&
      bal.asset_issuer === 'GBVYYQ7XXRZW6ZCNNCL2X2THNPQ6IM4O47HAA25JTAG7Z3CXJCQ3W4CD'
  );
  return real8 ? real8.balance : '0';
}

/**
 * Adds a trustline for the REAL8 asset.
 * @param accountId - Stellar public key
 * @param secret - Secret key of the account
 */
export async function addTrustline(accountId: string, secret: string) {
  const account = await server.loadAccount(accountId);
  const transaction = new TransactionBuilder(account, {
    fee: await server.fetchBaseFee(),
    networkPassphrase: 'Public Global Stellar Network ; September 2015',
  })
    .addOperation(Operation.changeTrust({ asset: REAL8 }))
    .setTimeout(30)
    .build();

  transaction.sign(Keypair.fromSecret(secret));
  await server.submitTransaction(transaction);
}

/**
 * Fetches available liquidity pools for the REAL8 asset.
 */
export async function fetchLiquidityPools() {
  const pools = await server.liquidityPools()
    .forAsset(REAL8)
    .call();
  return pools.records;
}
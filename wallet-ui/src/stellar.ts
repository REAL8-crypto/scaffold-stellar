import { Server, Asset } from '@stellar/stellar-sdk';

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
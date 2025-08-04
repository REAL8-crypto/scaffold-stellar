import storage from "./storage";
import {
  ISupportedWallet,
  StellarWalletsKit,
  sep43Modules,
  WalletNetwork,
} from "@creit.tech/stellar-wallets-kit";
import { Horizon } from "@stellar/stellar-sdk";
import { networkPassphrase, stellarNetwork, horizonUrl } from "../contracts/util";
import { fetchLiquidityPools } from '../stellar';

const kit: StellarWalletsKit = new StellarWalletsKit({
  network: networkPassphrase as WalletNetwork,
  modules: sep43Modules(),
});

export const connectWallet = async () => {
  await kit.openModal({
    modalTitle: "Connect to your wallet",
    onWalletSelected: (option: ISupportedWallet) => {
      const selectedId = option.id;
      kit.setWallet(selectedId);

      // Now open selected wallet's login flow by calling `getAddress` --
      // Yes, it's strange that a getter has a side effect of opening a modal
      void kit.getAddress().then((address) => {
        // Once `getAddress` returns successfully, we know they actually
        // connected the selected wallet, and we set our localStorage
        if (address.address) storage.setItem("walletId", selectedId);
        else storage.setItem("walletId", "");
      });
    },
  });
};

export const disconnectWallet = async () => {
  await kit.disconnect();
  storage.removeItem("walletId");
};

export const fetchBalance = async (address: string) => {
  const horizon = new Horizon.Server(horizonUrl, {
    allowHttp: stellarNetwork === "LOCAL",
  });

  const { balances } = await horizon.accounts().accountId(address).call();
  return balances;
};

export type Balance = Awaited<ReturnType<typeof fetchBalance>>[number];

export const wallet = kit;

// Example function to get liquidity pools
export async function getREAL8LiquidityPools() {
  return await fetchLiquidityPools();
}
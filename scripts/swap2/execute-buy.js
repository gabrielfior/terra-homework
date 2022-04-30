import { client, wallets } from '../library.js';

import {
  MsgExecuteContract,
  Coins, Coin,
} from "@terra-money/terra.js";

const contract = "terra1qcsq770f3hx2g2gsy3y5lqnrrwett9uzplj226";
const wallet = wallets.wallet_homework;


const amount = (0.5 * 1e6).toFixed(0);

const msg = new MsgExecuteContract(
  wallet.key.accAddress,
  contract,
  {
    buy: {},
  },
  [new Coin("uluna",  amount)],
);

const tx = await wallet.createAndSignTx({ msgs: [msg] });
const result = await client.tx.broadcast(tx);

console.log(result);
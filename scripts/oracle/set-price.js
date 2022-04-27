

import { client, wallets } from '../library.js';

import {
  MsgExecuteContract,
  Coins
} from "@terra-money/terra.js";

const oracleAddress = "terra15secglerg8y5setamsws3qnu4fv2ns7200za5j";
const wallet = wallets.wallet_homework;
const price = 28;

const msg = new MsgExecuteContract(
  wallets.wallet_homework.key.accAddress,
  oracleAddress,
  {
    "update_price": {
      "price": price,
    },
  }
);

const tx = await wallet.createAndSignTx({ msgs: [msg] });
const result = await client.tx.broadcast(tx);

console.log(result);
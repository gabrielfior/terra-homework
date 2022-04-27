import { client, wallets } from '../library.js';

import {
  MsgExecuteContract,
  Coins
} from "@terra-money/terra.js";

const contract = "terra1qzsr47twxtnh5wrsevw4r7acwg0hyrpgqyxfk3";
const wallet = wallets.wallet_homework;
const recipient = wallets.wallet1;


const amount = (0.5 * 1e6).toFixed(0);

const msg = new MsgExecuteContract(
  wallets.wallet_homework.key.accAddress,
  contract,
  {
    "mint": {
      "recipient": recipient.key.accAddress,
      "amount": (10 * 1e6).toFixed(0),
    },
  }
);

const tx = await wallet.createAndSignTx({ msgs: [msg] });
const result = await client.tx.broadcast(tx);

console.log(result);
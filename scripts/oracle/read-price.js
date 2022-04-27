import { client, wallets } from '../library.js';


import {
  MsgExecuteContract,
  MnemonicKey,
  Coins,
  LCDClient,
} from "@terra-money/terra.js";

const oracleAddress = "terra15secglerg8y5setamsws3qnu4fv2ns7200za5j";
const walletAddress = wallets.wallet_homework.key.accAddress;

const response = await client.wasm.contractQuery(oracleAddress,
   { query_price: { }});

console.log(response);


import { client, wallets } from '../library.js';

import {
  MsgExecuteContract,
  MnemonicKey,
  Coins,
  LCDClient,
} from "@terra-money/terra.js";

const cw20Contract = "terra1qzsr47twxtnh5wrsevw4r7acwg0hyrpgqyxfk3";
const walletAddress = wallets.wallet_homework.key.accAddress;

const response = await client.wasm.contractQuery(cw20Contract, { balance: { address: walletAddress }});

console.log(response);


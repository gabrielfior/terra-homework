import { Coin } from '@terra-money/terra.js';

module.exports = ({ wallets, refs, config, client }) => ({
  
  getCount: () => client.query("counter", { get_count: {} }),
  increment: (signer = wallets.validator) =>
    client.execute(signer, "counter", { increment: {} }),
  mint: (signer = wallets.homework) =>
    client.execute(signer, "cw20_token", { mint: { "recipient": "terra1qcsq770f3hx2g2gsy3y5lqnrrwett9uzplj226", "amount": "1000000000" } }),
  balance: () => client.query("cw20_token", { balance: { "address": "terra1qcsq770f3hx2g2gsy3y5lqnrrwett9uzplj226" } }),
  swap: () => client.execute(signer = wallets.homework, "swap", { buy: {} },
    // Send Luna with this execute message.
    new terraJs.Coins({ uluna: 1000000 })
  ),
  getPrice: () => client.query("oracle", { query_price: {} }),
  updatePrice: (signer = wallets.homework) =>
    client.execute(signer, "oracle", { update_price: { "price": 102 } }),
});




#Vesting---solana

This contract Implements Vesting functionality in solana with dynamic tge for every beneficiary followed by Linear vesting.Contract is written using anchor framework.

Deployed Program Address(Devnet) : AFLwi1VLdGgtHYmxdg2EeqkYvv2oMWwJE4FpTbQfroL1

Steps To use 

1.clone this repo.
```
git clone https://github.com/Maverick9081/Vesting---solana.git
```

2. install dependencies.
```
cd Vesting---solana
npm i
```

3. New to solana? follow this guide  :  https://careful-narcissus-91d.notion.site/Solana-7cceb917bce44716a18b8a36e0a7d9bd

4.Build the Program
```
anchor build
```

5. Configue `Anchor.toml` file for default wallets and cluster settings and program address.

6. Run the test
```
anchor test
```


For Web 3 scripts check the `test.js` file


Refs :
https://github.com/Bonfida/token-vesting

https://github.com/mralbertchen/solana-linear-vesting
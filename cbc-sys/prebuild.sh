[ -e coinor ] && rm -rf coinor
mkdir -p coinor
cd coinor

git clone https://github.com/coin-or/Cbc.git --branch releases/2.10.5
git clone https://github.com/coin-or/Cgl.git --branch releases/0.60.3
git clone https://github.com/coin-or/Clp.git --branch releases/1.17.6
git clone https://github.com/coin-or/CoinUtils.git --branch releases/2.11.4
git clone https://github.com/coin-or/Osi.git --branch releases/0.108.6

cd Cbc && git apply ../../cbc.patch
cd ../Clp && git apply ../../clp.patch

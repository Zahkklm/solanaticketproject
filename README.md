# Solana Etkinlik Bileti Sistemi

Bu proje, etkinlik yönetimini sağlayan bir Solana akıllı sözleşmesidir (program). Etkinlik oluşturma, bilet basma ve SPL token kullanarak bilet transferi gibi işlevler içerir. Program, Anchor framework kullanılarak geliştirilmiştir.

## Özellikler

- **Etkinlik Oluşturma:** Kullanıcıların etkinlik adı, tarihi, bilet fiyatı ve toplam bilet sayısı gibi ayrıntılarla yeni bir etkinlik oluşturmasını sağlar.
- **Bilet Basma:** SPL token transferi ile bilet satın almayı sağlar; etkinliğin token hesabından alıcının token hesabına bilet transferi yapılır.
- **Etkinlik Ayrıntılarını Görüntüleme:** Etkinlik ayrıntılarını depolar ve erişim sağlar.

## Gereksinimler

- [Rust](https://www.rust-lang.org/): Solana programını oluşturmak ve dağıtmak için gereklidir.
- [Anchor](https://project-serum.github.io/anchor/): Solana program geliştirme çerçevesi.
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools): Solana blok zinciri ile etkileşim kurmak için komut satırı aracı.
- [Node.js](https://nodejs.org/): Betikleri çalıştırmak ve Solana programı ile etkileşim kurmak için gereklidir.

## Kurulum

(WSL Ubuntu kullanılarak test edilmiştir.)

1. **Rust ve Anchor'ı Kurma**

    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source $HOME/.cargo/env
    cargo install anchor-cli --locked
    ```

2. **Projeyi klonlamak için**

    ```bash
    git clone https://github.com/Zahkklm/solanaticketproject
    cd solanaticketproject
    ```

3. **Build Etmek için**

    ```bash
    anchor build
    ```

4. **Solana CLI'yi Ayarlayın**

    - Eğer Solana CLI yüklü değilse, yükleyin.
    - Solana CLI'yi testnet veya devnet kullanacak şekilde yapılandırın.

    ```bash
    solana config set --url https://api.devnet.solana.com
    ```

## Kullanım

### 1. Yeni Bir Etkinlik Oluşturma

Akıllı sözleşmeyi Solana blok zincirine dağıtın ve ardından aşağıdaki komutu kullanarak bir etkinlik oluşturun.

```bash
anchor deploy

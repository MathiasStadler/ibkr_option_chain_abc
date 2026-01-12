# project path
<!-- keep the format -->
## init new project folder
<!-- KtF-->
```bash <!-- markdownlint-disable-line code-block-style -->
project_name="yfinance-rs-try-out"
echo ${project_name} 
# cd && mkdir <project_name> folder> && cd $_
# command 'cd' change to home folder from logged in user
# command 'mkdir' create the DIRECTORY(ies), if they do not already exist
# command `cd` <folder>`change to the folder
# command '$_' last argument of last command
cd && mkdir ${project_name} && cd $_

```
<!-- KtF -->
## Create a new rust based project inside the previously generated folder and update the rust binary's system wide to the last stable version
<!-- KtF -->
```bash <!-- markdownlint-disable-line code-block-style -->
touch project_path.md \
&& touch README.md \
&& touch FROM_HERE.md \
&& ln -s README.md README \
&& mkdir -p img \
&& curl --create-dirs --output-dir img -O  "https://raw.githubusercontent.com/MathiasStadler/link_symbol_svg/refs/heads/main/link_symbol.svg" \
&& echo "[1]: ./img/link_symbol.svg " >>project_path.md \
&& echo "[1]: ./img/link_symbol.svg " >>FROM_HERE.md \
&& echo "[1]: ./img/link_symbol.svg " >>README.md \
&& cargo init "." \
&& cargo add rustfmt \
&& rustup component add rustfmt \
&& mkdir examples \
&& cp src/main.rs examples/example.rs \
&& sed -i -e 's/world/example/g' examples/example.rs \
&& rustup show \
&& rustup check \
&& rustup toolchain uninstall stable \
&& rustup toolchain install stable \
&& export RUSTC_WRAPPER=sccache \
&& cargo list --update \
&& rustup update  --force \
&& rustup show \
&& rustup override set stable  \
&& cargo list --update \
&& cargo clippy \
&& cargo fmt --verbose \
&& cargo fix --workspace \
&& mkdir tests \
&& cargo build \
&& cargo run \
&& cargo run --example example
```

❌ rust-toolchain - Create rust-toolchain on init project
<!-- KtF-->
## Show which toolchain is active
<!-- KtF-->
```bash <!-- markdownlint-disable-line code-block-style -->
rustup show
# or better
rustup show |sed -n '/active toolchain/,/^$/p'
```
<!-- KtF-->
## Switch/activate toolchain
<!-- KtF-->
```bash <!-- markdownlint-disable-line code-block-style -->
#e.g. nightly
rustup override set nightly
# e.g stable
rustup override set stable
```
<!-- KtF-->
## Show which toolchain is active - we need stable
<!-- keep the format -->
```bash <!-- markdownlint-disable-line code-block-style -->
rustup show
# or better
rustup show |sed -n '/active toolchain/,/^$/p'
```
<!-- keep the format -->
## Switch between Rust toolchains - Check again - See previous step [![alt text][1]](https://stackoverflow.com/questions/58226545/how-to-switch-between-rust-toolchains)
<!-- keep the format -->
```bash <!-- markdownlint-disable-line code-block-style -->
rustup override set nightly
rustup override set stable
# another toolchain by name
rustup override set 1.85.0-x86_64-unknown-linux-gnu
```
<!-- keep the format -->
## start gateway
<!-- keep the format -->
```bash <!-- markdownlint-disable-line code-block-style -->
cd /home/trapapa/clientportal.gw
./bin/run.sh root/conf.yaml
```
<!-- keep the format -->
## check gate way is running on port XXX
<!-- keep the format -->
```bash <!-- markdownlint-disable-line code-block-style -->
sudo ss -lptn 'sport = :5000'
# ps -ef |grep <pid>
ps -ef |grep 63248

```
<!-- keep the format -->

https://localhost:5000/api

sudo ss -lptn 'sport = :5000'

## gateway status [![alt text][1]](https://github.com/Voyz/ibeam/wiki/TLS-Certificates-and-HTTPS)
<!-- keep the format -->
- cURL accepts --insecure or -k flags that can be used to ignore verifying certificates. See cURL documentation for more
<!-- keep the format -->
```bash <!-- markdownlint-disable-line code-block-style -->
curl -X GET "https://localhost:5000/v1/api/iserver/auth/status" -k
```
<!-- keep the format -->

>[!NOTE]
>Symbol to mark web external links [![alt text][1]](./README.md)
<!-- spell-checker: disable  -->
<!-- spell-checker: disable  -->
<!-- keep the format -->
<!-- make folder and download the link sign vai curl -->
<!-- mkdir -p img && curl --create-dirs --output-dir img -O  "https://raw.githubusercontent.com/MathiasStadler/link_symbol_svg/refs/heads/main/link_symbol.svg"-->
<!-- Link sign - Don't Found a better way :-( - You know a better method? - **send me a email** -->
[1]: ./img/link_symbol.svg
<!-- keep the format -->
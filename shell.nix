{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    qemu
    OVMF
    cargo
  ];

  shellHook = ''
    mkdir -p drive
    mkdir -p drive/esp
    mkdir -p drive/esp/efi/boot
    echo "checking OVMF files" 
    # this was hell to find
    ovmf_code="$(ls /nix/store | grep "qemu" | awk '{print "/nix/store/" $0; "\0"}' | tr '\n' '\0' | find -files0-from - -type f | grep --max-count=1 "edk2-x86_64-code\.fd")"
    echo "$ovmf_code"
    cp -i "$ovmf_code" "./drive/."
    echo "pushd \`pwd\`; cd $(dirname $0); exec qemu-system-x86_64 -enable-kvm -display gtk -drive if=pflash,format=raw,readonly=on,file=edk2-x86_64-code.fd -drive format=raw,file=fat:rw:esp; popd" > ./drive/run_qemu.sh
  '';
}


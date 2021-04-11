Vagrant.configure("2") do |config|
  config.vm.box = "yzgyyang/macOS-10.14"

  config.vm.provider "virtualbox" do |vb|
    vb.gui = true
    vb.check_guest_additions = false
    vb.memory = "8192"
    vb.cpus = 4

    # I need a serial number to do MDM stuff
    # ref https://old.reddit.com/r/Intune/comments/e4tibs/macos_profile_installation_failing/f9fypjp/
    # also ref https://simplemdm.com/apple-dep-vmware-parallels-virtualbox/
    vb.customize ["setextradata", :id, "VBoxInternal/Devices/efi/0/Config/DmiSystemSerial", "FVFZX1A1JYW0"]
    vb.customize ["setextradata", :id, "VBoxInternal/Devices/efi/0/Config/DmiSystemProduct", "Macmini8,1"]

    # According to this answer it needs to be minimal for Mac OS guests
    # https://superuser.com/a/1009306/79714
    vb.customize ["modifyvm", :id, "--paravirtprovider", "minimal"]

    # According to this article the graphics controller should be VMSVGA
    # https://www.geekrar.com/how-to-install-guest-tool-on-macos-on-virtualbox/
    vb.customize ["modifyvm", :id, "--graphicscontroller", "vmsvga"]
    vb.customize ["modifyvm", :id, "--vram", "128"]
  end

  config.vm.synced_folder ".", "/vagrant", disabled: true

  config.vm.provision "shell", privileged: false, inline: <<-SHELL
    brew cask install google-chrome intune-company-portal
  SHELL

  # This script seem to setup a lot, refrence that if anything is needed:
  # https://github.com/myspaghetti/macos-virtualbox/blob/master/macos-guest-virtualbox.sh
end

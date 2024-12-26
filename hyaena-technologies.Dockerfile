FROM amd64/debian:latest

WORKDIR /hyaena-technologies

COPY ./ ./

RUN apt update && apt -y full-upgrade 
RUN apt -y install apt-transport-https iptables firewalld nftables
RUN wget -qO- https://dl-ssl.google.com/linux/linux_signing_key.pub \ | gpg  --dearmor -o /usr/share/keyrings/dart.gpg
RUN echo 'deb [signed-by=/usr/share/keyrings/dart.gpg arch=amd64] https://storage.googleapis.com/download.dartlang.org/linux/debian stable main' \ | tee /etc/apt/sources.list.d/dart_stable.list
RUN apt update && apt -y install dart
RUN echo 'export PATH="$PATH:~/.pub-cache/bin"' >> ~/.bashrc && sudo echo 'export PATH="$PATH:~/.pub-cache/bin"' >> /etc/skel/.bashrc
RUN echo 'export PATH="$PATH:/usr/bin"' >> ~/.bashrc && echo 'export PATH="$PATH:/usr/bin"' >> /etc/skel/.bashrc
RUN echo 'export PATH="$PATH:/usr/local/bin"' >> ~/.bashrc && echo 'export PATH="$PATH:/usr/local/bin"' >> /etc/skel/.bashrc
RUN echo 'export PATH="$PATH:/usr/include"' >> ~/.bashrc && echo 'export PATH="$PATH:/usr/include"' >> /etc/skel/.bashrc
RUN echo 'export PATH="$PATH:/usr/local/include"' >> ~/.bashrc && echo 'export PATH="$PATH:/usr/local/include"' >> /etc/skel/.bashrc  
RUN wget https://go.dev/dl/go1.23.4.linux-amd64.tar.gz && tar --extract --file ./*.gz --verbose
RUN mv ./go1.23.4.linux-amd64/go /usr/bin
RUN echo 'export PATH="$PATH:/usr/bin/go/bin"' >> ~/.bashrc && echo 'export PATH="$PATH:/usr/bin/go/bin"' >> /etc/skel/.bashrc
RUN add-apt-repository ppa:maveonair/helix-editor && apt update
RUN apt -y install ed sed nano vim neovim helix
RUN dart info
RUN dart pub global activate webdev
RUN dart pub upgrade
RUN dart pub deps
RUN dart pub outdated
RUN dart analyze lib
RUN dart fix lib --dry-run
RUN dart fix lib --apply
RUN dart format lib

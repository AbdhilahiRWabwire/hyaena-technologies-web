FROM amd64/debian:latest

WORKDIR /hyaena_technologies

COPY ./ ./

RUN apt update && apt -y full-upgrade 
RUN apt -y install apt-transport-https
RUN wget -qO- https://dl-ssl.google.com/linux/linux_signing_key.pub \ | gpg  --dearmor -o /usr/share/keyrings/dart.gpg
RUN echo 'deb [signed-by=/usr/share/keyrings/dart.gpg arch=amd64] https://storage.googleapis.com/download.dartlang.org/linux/debian stable main' \ | tee /etc/apt/sources.list.d/dart_stable.list
RUN apt update && apt -y install dart
RUN echo 'export PATH="$PATH:/usr/bin"' >> ~/.bashrc && echo 'export PATH="$PATH:/usr/include"' >> ~/.bashrc && echo 'export PATH="$PATH:/usr/local/include"' >> ~/.bashrc
RUN echo 'export PATH="$PATH:/usr/bin"' >> /etc/skel/.bashrc && echo 'export PATH="$PATH:/usr/include"' >> /etc/skel/.bashrc && echo 'export PATH="$PATH:/usr/local/include"' >> /etc/skel/.bashrc
RUN wget https://ziglang.org/download/0.13.0/zig-linux-x86_64-0.13.0.tar.xz && tar --extract --file ./*.xz --verbose
RUN mv ./zig-linux-x86_64-0.13.0 ./zig && mv ./zig /usr/bin
RUN echo 'export PATH="$PATH:/usr/bin/zig"' >> ~/.bashrc && echo 'export PATH="$PATH:/usr/bin/zig"' >> /etc/skel/.bashrc
RUN zig version
RUN dart info
RUN dart pub global activate webdev
RUN dart pub upgrade
RUN dart pub deps
RUN dart pub outdated
RUN dart analyze lib
RUN dart fix lib --dry-run
RUN dart fix lib --apply
RUN dart format lib

# frameworktools/ci image

# docker buildx create --name mybuilder
# docker buildx use mybuilder
# docker buildx build --push --platform linux/amd64 -t frameworktools/ci:latest .

# docker build -t frameworktools/ci:latest
# docker push frameworktools/ci:latest


# Dockerfile for the CircleCI Image Builder
# This is not the dockerfile for the app

FROM node:17-alpine

RUN apk update && apk add \
		tar \
		ca-certificates \
        python3 \
        py3-pip \
		curl \
		git \
		docker \
        gettext
		# libffi-dev \
		# llvm \
		# make \
		# tk-dev \
		# wget

# Install kubectl
RUN curl -LO "https://dl.k8s.io/release/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/linux/amd64/kubectl"
RUN install -o root -g root -m 0755 kubectl /usr/local/bin/kubectl
RUN kubectl version --client

# Install python
RUN pip3 install --upgrade pip && \
    pip3 install \
        awscli

RUN rm -rf /var/cache/apk/*
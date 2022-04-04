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
		docker
		# libffi-dev \
		# llvm \
		# make \
		# tk-dev \
		# wget

# Install python
RUN pip3 install --upgrade pip && \
    pip3 install \
        awscli

RUN rm -rf /var/cache/apk/*
# Dockerfile for the CircleCI Image Builder
# This is not the dockerfile for the app

FROM node:14-alpine

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
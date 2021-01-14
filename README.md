# Alexandrie - Backend

This repository holds the backend part of the Alexandrie project.

## Alexandrie

Alexandrie is the common account manager used by Tiwind Software applications.

You can think of it as a semi-toy project, as it will be used to power other toy projects but has to do so in a
consistent manner, with a stable implementation and API, and with the goal of being performant, stable and robust. The
medium-term target platform is POSIX-compliant single servers, with the ability to run as a distributed Kubernetes
service as the long-term goal.

As a user manager, Alexandrie assumes the following responsibilities:

* Offer account creation and management for users, as well as legal and technical entities, both as a self-service and
  in a managed manner.
* Expose an API that can be used by authorized applications to use Alexandrie's capabilities.
* Handle authentication, authorization and privilege rules and groups.
* Answer the needs of user data governance in a cross-application manner, non-exhaustively including regulatory
  compliance or data security and quality concerns.

## Developer's guide

### Running Alexandrie locally

The following dependencies are required to run Alexandrie locally:

* A Rust compiler (compatible with edition 2018) and Cargo,
* A PostgreSQL (>= 13) server.

While setting up and running a PostgreSQL server directly on the host machine is possible, this is currently unsupported
and it is recommended to use the development stack instead.

#### Running the development stack

The development stack has the following dependencies:

* A Docker client and server (daemon),
* `docker-compose`

With those requirements met, the development stack can be started using the following command at the root of this
project:

```shell
# Will create appropriate volumes and containers
# for the development stack and run them.
docker-compose dev/docker-compose.yml up
```

A few additional files need to be created in order to run Alexandrie, all relative to project root:

* `./.env`, which is used for telling the tooling how to connect to the database. It should contain a `DATABASE_URL`
environment variable of the following format (replace `docker-daemon-host` with an IP or hostname pointing to your
  docker daemon):
```shell
DATABASE_URL=postgresql://alexandrie:password@docker-daemon-host/alexandrie
```
* `./appconfig.yml` offers defaults that assume that your docker daemon is running on `localhost`. If this is not the case, an additional configuration
file `./appconfig-dev.yml` can be created:
```yaml
database:
  host: docker-daemon-host
```

#### Running Alexandrie itself

Once the above dependencies are met, Alexandrie can simply be run using `cargo run`. On first run, the database will automatically
be set up and the application will listen for HTTP connections on `localhost:8080`.

In order to verify that Alexandrie is running, you can use the health endpoint: `GET http://localhost:8080/v1/health`.

Note that if your docker daemon is not running on `localhost` and you created a `appconfig-dev.yml` file while setting up
the development stack, it will have to be passed as a parameter to Cargo:
```shell
cargo run -- --configuration-file appconfig-dev
```
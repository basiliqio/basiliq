<h1 align="center"> Basiliq </h1>

<h4 align="center"><b>Exposing a <em>Postgres</em> via a REST API that respects <a href="https://jsonapi.org/format/">JSON:API</a>.
<br>
All in all, a tasty API.
</b>
</h4>

<a href="https://gitlab.com/basiliqio/basiliq/-/pipelines" alt="Gitlab pipeline status">
  <img src="https://img.shields.io/gitlab/pipeline/basiliqio/basiliq/main">
</a>
<a href="https://codecov.io/gl/basiliqio/basiliq" alt="Codecov">
  <img src="https://img.shields.io/codecov/c/github/basiliqio/basiliq?token=HLjRazfpcL">
</a>
<a href="https://crates.io/crates/basiliq" alt="Crates.io version">
  <img src="https://img.shields.io/crates/v/basiliq">
</a>
<a href="https://crates.io/crates/basiliq" alt="Crates.io license">
  <img src="https://img.shields.io/crates/l/basiliq?label=license">
</a>
<a href="https://docs.rs/basiliq" alt="Docs.rs">
  <img src="https://docs.rs/basiliq/badge.svg">
</a>
<img align="right" width="50%" src="assets/logos/LOGO_Basiliq_large.svg"></div>

- [What is Basiliq](#what-is-basiliq)
- [Quickstart](#quickstart)
	- [Ready to use example](#ready-to-use-example)
	- [Running locally](#running-locally)
- [Understanding the API](#understanding-the-api)
	- [How to query](#how-to-query)
	- [Example requests](#example-requests)
- [The configuration](#the-configuration)
	- [Generation](#generation)
	- [What's in there](#whats-in-there)
	- [Checking the configuration](#checking-the-configuration)
- [Testing](#testing)


## What is Basiliq

Basiliq is a **very alpha** REST API that abstracts the need to write CRUD methods by exposing a standardized API to interact with a [Postgres](https://www.postgresql.org/) database

It respects the established [JSON:API](https://jsonapi.org/format/)
specifications. Written in [Rust](https://www.rust-lang.org/fr), it tries to conciliate performance with stability.

## Quickstart

### Ready to use example

You could try out the API already deployed on [Heroku](https://www.heroku.com/).

For instance:

```sh

# For a very simple example
curl 'http://demo.basiliq.io/public__peoples'

# For a more complexe example
curl 'http://demo.basiliq.io/public__peoples?include=public__articles,public__comments&fields\[public__comments\]='

```
### Running locally

One can install _basiliq_ through docker. An example `docker-compose` script is available at the root of the repository. To start it:

```sh
# To start
docker-compose -f docker-compose.example.yml up -d

# At that point the database is empty,
# you can populate it with the following script
curl -L https://gitlab.com/basiliqio/basiliq_db_test_utils/-/jobs/artifacts/main/raw/basiliq_test.dump\?job\=pack_test_migrations | PGHOST=localhost PGUSER=postgres PGPASS=postgres psql

# Then you can restart the basiliq server so it rescans the database
docker-compose -f docker-compose.example.yml restart basiliq

# To stop
docker-compose -f docker-compose.example.yml down
```

## Understanding the API

### How to query

In the future, there should be a way to generate an [OpenApi](https://swagger.io/specification/) document to view exactly how the _API_ is accessible.

The _API_ response respects the [JSON:API specifications](https://jsonapi.org/format/).

By default, the endpoint are exposed in the format `schema__table`
(i.e. for a table `peoples` in the `public` schema, the endpoint would be `public__table`).

By modifying the configuration one could change how those endpoints are exposed.

### Example requests

<details>
<summary>Creating request</summary>

Notice the lack of id in the request.

Also, in the response, the fields that were not included are set to their default

```http
POST /public__peoples HTTP/1.1
Host: demo.basiliq.io
User-Agent: curl/7.76.1
Content-Type:application/vnd.api+json
Accept: application/json, */*
Content-Length: 174

{
    "data": {
        "type": "public__peoples",
        "attributes": {
            "first-name": "Somebody",
            "last-name": "Once_told_me_the_world",
            "gender": "F",
            "twitter": "@allstars"
        }
    }
}

HTTP/1.1 201 Created
Connection: keep-alive
Content-Type: application/vnd.api+json
Content-Length: 224
Date: Sun, 02 May 2021 20:20:52 GMT

{
    "jsonapi": {
        "version": "1.0"
    },
    "data": {
        "type": "public__peoples",
        "id": "d14e1928-9cae-441c-945d-144ebe6c94c8",
        "attributes": {
            "age": null,
            "first-name": "Somebody",
            "gender": "F",
            "last-name": "Once_told_me_the_world",
            "twitter": "@allstars"
        }
    }
}
```
</details>

<details>
<summary>Simple fetching request</summary>

```http
GET /public__peoples HTTP/1.1
Host: demo.basiliq.io
User-Agent: curl/7.76.1
Accept: application/json, */*

HTTP/1.1 200 OK
Connection: keep-alive
Content-Type: application/vnd.api+json
Content-Length: 598
Date: Sun, 02 May 2021 20:13:47 GMT

{
    "jsonapi": {
        "version": "1.0"
    },
    "data": [
        {
            "type": "public__peoples",
            "id": "1649b1e9-8a5f-4f52-b331-c07ce3bccc6f",
            "attributes": {
                "age": 22,
                "first-name": "Francis",
                "gender": "M",
                "last-name": "Le Roy",
                "twitter": null
            }
        },
        {
            "type": "public__peoples",
            "id": "777cc565-c66b-4942-ab44-8fc5f194b804",
            "attributes": {
                "age": 34,
                "first-name": "Somebody",
                "gender": "F",
                "last-name": "Wuhu",
                "twitter": "@randomhandle"
            }
        },
        {
            "type": "public__peoples",
            "id": "961e543a-4b22-4d48-a8e5-c1eafada950f",
            "attributes": {
                "age": null,
                "first-name": "AAAAAAAA",
                "gender": null,
                "last-name": "BBBBBBBBB",
                "twitter": null
            }
        }
    ]
}
```
</details>

<details>
<summary>Fetching requests including relationships and sparsing fields</summary>

You can find the attributes of the objects in the `relationships` key of each main resource in the `included` key below.

Notice that the comments object have only ids, because all of their fields have been un-selected via the `fields[public__comments]=` query parameter.

```http
GET /public__peoples?include=public__articles,public__comments&fields[public__comments]= HTTP/1.1
Host: demo.basiliq.io:80
User-Agent: curl/7.76.1
Accept: application/json, */*

HTTP/1.1 200 OK
content-type: application/vnd.api+json
content-length: 1879
date: Sun, 02 May 2021 20:08:12 GMT

{
    "jsonapi": {
        "version": "1.0"
    },
    "data": [
        {
            "type": "public__peoples",
            "id": "1649b1e9-8a5f-4f52-b331-c07ce3bccc6f",
            "attributes": {
                "age": 22,
                "first-name": "Francis",
                "gender": "M",
                "last-name": "Le Roy",
                "twitter": null
            },
            "relationships": {
                "public__articles": {
                    "data": [
                        {
                            "type": "public__articles",
                            "id": "fdf715dd-8772-498c-8196-6f4ccb64edef"
                        },
                        {
                            "type": "public__articles",
                            "id": "2dbf5d1a-b029-4456-af6b-339c75b1089c"
                        }
                    ]
                },
                "public__comments": {
                    "data": [
                        {
                            "type": "public__comments",
                            "id": "59f58abd-c9db-4074-9c34-ac33e9c838ce"
                        },
                        {
                            "type": "public__comments",
                            "id": "c2add83b-6f58-45a2-bf62-3ebc05c46192"
                        }
                    ]
                }
            }
        },
        {
            "type": "public__peoples",
            "id": "777cc565-c66b-4942-ab44-8fc5f194b804",
            "attributes": {
                "age": 34,
                "first-name": "Somebody",
                "gender": "F",
                "last-name": "Wuhu",
                "twitter": "@randomhandle"
            },
            "relationships": {
                "public__articles": {
                    "data": {
                        "type": "public__articles",
                        "id": "46c4fe50-8c56-4f26-935e-56ccfa496bb5"
                    }
                },
                "public__comments": {
                    "data": {
                        "type": "public__comments",
                        "id": "6ae9938f-d490-4707-b138-770c2a52465f"
                    }
                }
            }
        },
        {
            "type": "public__peoples",
            "id": "961e543a-4b22-4d48-a8e5-c1eafada950f",
            "attributes": {
                "age": null,
                "first-name": "AAAAAAAA",
                "gender": null,
                "last-name": "BBBBBBBBB",
                "twitter": null
            }
        }
    ],
    "included": [
        {
            "type": "public__articles",
            "id": "2dbf5d1a-b029-4456-af6b-339c75b1089c",
            "attributes": {
                "body": "Yeah I know ! Right ?!",
                "title": "Oh my g**"
            }
        },
        {
            "type": "public__articles",
            "id": "46c4fe50-8c56-4f26-935e-56ccfa496bb5",
            "attributes": {
                "body": "They feast on the blood of the departed draw their powers",
                "title": "Why devs require sacrifices"
            }
        },
        {
            "type": "public__articles",
            "id": "fdf715dd-8772-498c-8196-6f4ccb64edef",
            "attributes": {
                "body": "Yes",
                "title": "How to dead"
            }
        },
        {
            "type": "public__comments",
            "id": "59f58abd-c9db-4074-9c34-ac33e9c838ce"
        },
        {
            "type": "public__comments",
            "id": "6ae9938f-d490-4707-b138-770c2a52465f"
        },
        {
            "type": "public__comments",
            "id": "c2add83b-6f58-45a2-bf62-3ebc05c46192"
        }
    ]
}
```
</details>

<details>
<summary>Updating request</summary>

Notice that attributes that were not included in the `PATCH` request are not nulled.

```http
PATCH /public__peoples/777cc565-c66b-4942-ab44-8fc5f194b804 HTTP/1.1
Host: demo.basiliq.io
User-Agent: curl/7.76.1
Content-Type:application/vnd.api+json
Accept: application/json, */*
Content-Length: 204

{
    "data": {
        "type": "public__peoples",
        "id": "777cc565-c66b-4942-ab44-8fc5f194b804",
        "attributes": {
            "first-name": "NotTheOriginalFirstName",
            "last-name": "NotTheOriginalLastName"
        }
    }
}

HTTP/1.1 200 OK
Connection: keep-alive
Content-Type: application/vnd.api+json
Content-Length: 260
Date: Sun, 02 May 2021 20:24:50 GMT

{
    "jsonapi": {
        "version": "1.0"
    },
    "data": {
        "type": "public__peoples",
        "id": "777cc565-c66b-4942-ab44-8fc5f194b804",
        "attributes": {
            "age": 34,
            "first-name": "NotTheOriginalFirstName",
            "gender": "F",
            "last-name": "NotTheOriginalLastName",
            "twitter": "@randomhandle"
        }
    }
}

```
</details>

<details>
<summary>Deleting request</summary>

```http
DELETE /public__peoples/777cc565-c66b-4942-ab44-8fc5f194b804 HTTP/1.1
Host: demo.basiliq.io
User-Agent: curl/7.76.1
Accept: application/json, */*

HTTP/1.1 200 OK
Connection: keep-alive
Content-Type: application/vnd.api+json
Content-Length: 41
Date: Sun, 02 May 2021 20:25:54 GMT

{
    "jsonapi": {
        "version": "1.0"
    },
    "data": null
}
```
</details>


## The configuration

### Generation 
Typically, one would first need to create a configuration, however this is
not mandatory to run _basiliq_.

To create a configuration, one can use : 

```sh
basiliq config generate
```

It would generate a file called `basiliq_config.yaml` in the current working directory.

### What's in there

This file would look like the following :

```yml
---
resources:                 # The list of resources
  public__articles:        # The name of a resource. *It can be changed*
    target:                # The identifier object of the resource
      schema: public       # The schema this resource is bound to in the database
      table: articles      # The name of the table bound to this resource
    enabled: true          # `true` if this resource is enabled
    relationships:         # A list of relationships
      public__comments:    # Name of the relationship. *It can be changed*
        target:            # The identifier object of the resource
          schema: public   # The schema this relationship's resource is bound to in the database
          table: comments  # The name of the table bound to this relationship's resource
        enabled: true      # `true` if this relationship is enabled
        field: article     # The field on which this relationship is bound
      public__peoples:
        target:
          schema: public
          table: peoples
        through:           # For Many-to-Many relationships identify the bucket table
          schema: public   # The schema of the bucket resource
          table: comments  # The table of the bucket resource
          field: author    # The field on which that this relationship's resource is bound to the bucket resource
        enabled: true
        field: id
[...]
```

### Checking the configuration

After having generated the configuration, one might need to ensure its correct.

One could do that with the following command:

```sh
basiliq config check --input basiliq_config.yaml 
```

## Testing

To test this crate, you need to start a `Postgres` database and export the `DATABASE_URL` environment variable.

You can use the provided `docker-compose` plan

```sh
# To start the test database
docker-compose -f docker-compose.testing.yml up -d

# Don't forget to set the environment variable
export DATABASE_URL="postgres://postgres:postgres@localhost/postgres"

# Run the tests
cargo test

# To stop the test database
docker-compose -f docker-compose.testing.yml down -v
```

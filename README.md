# casbin-grpc

Casbin-gRPC provides gRPC interface for Casbin authorization which is implemented with Rust.

Casbin Server is the `Access Control as a Service (ACaaS)` solution based on [Casbin](https://github.com/casbin/casbin). It provides [gRPC](https://grpc.io/) interface for Casbin authorization.

## What is `Casbin Server`?

Casbin-Server is just a container of Casbin enforcers and adapters. Casbin-Server performs the policy enforcement check, which may take a fair amount of processing depending on the model and number of policies, interfacing to external data stores such as databases for policy data. Just like how native Casbin library works, each Casbin enforcer in Casbin-Server can use its own adapter, which is linked with external database for policy storage.

Of course, you can setup Casbin-Server together with your policy database in the same machine. But they can be separated. If you want to achieve high availability, you can use a Redis cluster as policy storage, then link Casbin-Server's adapter with it. In this sense, Casbin enforcer can be viewed as stateless component. It just retrieves the policy rules it is interested in (via sharding), does some computation and then returns `allow` or `deny`.

## Architecture

Casbin-Server uses the client-server architecture. Casbin-Server itself is the server (in Golang only for now). The clients for Casbin-Server are listed here:

| Language | Author                                                         | Client                                                      |
| -------- | -------------------------------------------------------------- | ----------------------------------------------------------- |
| Golang   | Casbin                                                         | https://github.com/casbin/casbin-go-client                  |
| Java     | [Accept008](https://github.com/Accept008)                      | https://github.com/Accept008/grpc-client                    |
| PHP      | Casbin                                                         | https://github.com/php-casbin/casbin-client                 |
| Golang   | [paysuper](https://github.com/paysuper)                        | https://github.com/paysuper/echo-casbin-middleware          |
| Python   | [@prathik-kaliyambath](https://github.com/prathik-kaliyambath) | https://github.com/prathik-kaliyambath/casbin-python-client |

Contributions for clients in other languages are welcome :)

## Getting Help

- [Casbin](https://github.com/casbin/casbin-rs)

## License

This project is under Apache 2.0 License. See the [LICENSE](LICENSE) file for the full license text.

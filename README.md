# Kraken-Orchestrator-Discovery

This is an orchestration discovery service for the [Kraken Project](https://github.com/ethanshry/Kraken).

Because Kraken has no built-in DNS services, this tool has been broken out in order to easily determine the IP address of the currently running Kraken Orchestrator. 

This tool relies on being able to find an open Kraken REST API Port. If this port changes, this utility will need to be updated, and if another service is using the same port, this tool could give an incorrect IP address.


Type-safe REST API
===============================

**Superseded by `Apivolve`_**

There are several features I want to explore in this repository:

- Define a typ-safe rest api using a trait (callee side is the server, caller side is the client).
- From just traits and macros, auto-generate rest api communication.
- Use the same traits to call directly (without rest calls) with minimal/no code changes.

**The code is not done yet, it is an ongoing experiment.**

Structure
-------------------------------

"User" code:

* **iface** The rest interface that client and server communicate through.
* **myserver** An example implementation of server logic.
* **myclient** An example implementation of client logic.

Binaries for testing:

* **serverbin** A server that can be reached through http and provides *myserver* (*iface* api).
* **clientbin** A client that does the client logic and calls *serverbin* through http.
* **directbin** A combination of client and server logic where calls happen natively instead of through rest.

This package's code:

* **apilib** The types and macros needed for defining and generating the api.
* **gen_rest_server** Wrapper derived from *iface* that makes *myserver* accessible through http api calls.
* **gen_rest_client** Wrapper derived from *iface* for the client that converts *iface* calls to api requests.


.. _`Apivolve`: https://github.com/mverleg/apivolve



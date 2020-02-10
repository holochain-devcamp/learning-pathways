# hUdemy
Udemy on Holochain.

hUdemy is a Peer to Peer education system based on Holochain. 


### Running a UI (or two)
To be able to run the UI and have a working version, follow the below steps:

* Navigate to the ui-folder
* Run the 'npm install' command

#### Run two agents for demo-ing purpose:
* Open terminal
* Navigate to ui-folder
* Run 'npm run demo'
* Open browser window and visit: http://localhost:8080
* Open second browser window and visit: http://localhost:8081

#### Run a single agent:
* Open terminal
* Navigate to ui-folder
* Run 'npm run hc:alice'
* Open another terminal
* Run 'npm run ui:alice'
* Open browser window and visit: http://localhost:8080

#### Frontend Stack
The front-end stack being used (see package.json):
* [LitElement](https://lit-element.polymer-project.org/)
* [GraphQL](https://graphql.org/)
* [ApolloClient](https://github.com/apollographql/apollo-client)
# hUdemy
Udemy on Holochain.

hUdemy is a Peer to Peer education system based on Holochain. 


### Running a UI (or two)
To be able to run the UI and have a working version, follow the below steps:

1. Navigate to the ui-folder
2. Run the 'npm install' command

#### Run two agents for demo-ing purpose:
1. Open terminal
2. Navigate to ui-folder
3. Run 'npm run demo'
4. Open browser window and visit: http://localhost:8080
5. Open second browser window and visit: http://localhost:8081

#### Run a single agent:
1. Open terminal
2. Navigate to ui-folder
3. Run 'npm run hc:alice'
4. Open another terminal
5. Run 'npm run ui:alice'
6. Open browser window and visit: http://localhost:8080

#### Frontend Stack
The front-end stack being used (see package.json):
* [LitElement](https://lit-element.polymer-project.org/)
* [GraphQL](https://graphql.org/)
* [ApolloClient](https://github.com/apollographql/apollo-client)
import gql from 'graphql-tag';

export const typeDefs = gql`
  directive @loadEntry on FIELD_DEFINITION

  type Course {
    id: ID!
    title: String!
    modules: [Module!]! @loadEntry
    teacher_address: ID!
    students: [ID!]!
  }

  type Module {
    id: ID!
    course_address: Course! @loadEntry
    title: String!
    contents: [Content!]! @loadEntry
  }

  type Content {
    id: ID!
    name: String!
    description: String!
    url: String!
  }

  type Query {
    courses(filter: String!): [Course!]! @loadEntry
    course(courseId: ID!): Course! @loadEntry
    myAddress: ID!
  }

  input ContentInput {
    name: String!
    description: String!
    url: String!
  }

  type Mutation {
    createCourse(title: String!): Course! @loadEntry
    updateCourse(courseId: ID!, title: String!, modulesIds: [ID!]!): Course!
      @loadEntry
    deleteCourse(courseId: ID!): Query
    createModule(courseId: ID!, title: String!): Course! @loadEntry
    updateModule(courseId: ID!, moduleId: ID!, title: String!): Course!
      @loadEntry
    deleteModule(courseId: ID!, moduleId: ID!): Course! @loadEntry
    createContent(courseId: ID!, moduleId: ID!, content: ContentInput!): Course! @loadEntry
    updateContent(courseId: ID!, contentId: ID!, content: ContentInput!): Course! @loadEntry
    deleteContent(courseId: ID!, contentId: ID!): Course! @loadEntry
    enrolInCourse(courseId: ID!): Course! @loadEntry
  }
`;

/*
 */

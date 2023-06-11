/// NB: The tryorama config patterns are still not quite stabilized.
/// See the tryorama README [https://github.com/holochain/tryorama]
/// for a potentially more accurate example


const path = require("path");

const {
  Orchestrator,
  Config,
  combine,
  singleConductor,
  localOnly,
  tapeExecutor
} = require("@holochain/tryorama");

process.on("unhandledRejection", error => {
  // Will print "unhandledRejection err is not defined"
  console.error("got unhandledRejection:", error);
});

const dnaPath = path.join(__dirname, "../dist/dna.dna.json");

const orchestrator = new Orchestrator({
  middleware: combine(
    tapeExecutor(require("tape")),
    localOnly
  )
});

const dna = Config.dna(dnaPath, "course_dna");
const conductorConfig = Config.gen(
  { course_dna: dna },
  {
    network: {
      type: "sim2h",
      sim2h_url: "ws://localhost:9000"
    },
    logger: Config.logger({ type: "error" }),
  }
);

// NOTE (e-nastasia): this scenario didn't have a description at all, so for the sake of consistency I've made one
// and numerated it 0 because it's the first one that is defined in this file.
// Please feel free to re-organize this numeration as appropriate
orchestrator.registerScenario("Scenario 0: User can create courses and they and other users can see them", async (s, t) => {
  const { alice, bob } = await s.players(
    { alice: conductorConfig, bob: conductorConfig },
    true
  );

  // Alice creates first course
  const course_addr_1 = await alice.call(
    "course_dna",
    "courses",
    "create_course",
    {
      title: "course for scenario 0",
      timestamp: 123
    }
  );

  console.log(course_addr_1);
  // we verify that first course was created succesfully
  t.ok(course_addr_1.Ok);
  await s.consistency();

  // Alice creates second course
  const course_addr_2 = await alice.call(
    "course_dna",
    "courses",
    "create_course",
    {
      title: "course for scenario 0",
      timestamp: 1234
    }
  );
  console.log(course_addr_2);
  // we verify that second course was created succesfully
  t.ok(course_addr_2.Ok);
  await s.consistency();

  // list all courses that belong to Alice
  const all_courses_alice = await alice.call("course_dna", "courses", "get_my_courses", {
  });

  // since Alice created 2 courses, we verify that this list contains 2 addresses
  t.true(all_courses_alice.Ok[0] != null);
  t.true(all_courses_alice.Ok[1] != null);

  // list all courses from perspective of other user Bob who didn't create any courses
  const all_courses_bob = await bob.call("course_dna", "courses", "get_all_courses", {
  });
  // even though Bob didn't create any courses, he should still see 2 courses (created by Alice)
  t.true(all_courses_bob.Ok[0] != null);
  t.true(all_courses_bob.Ok[1] != null);

  await s.consistency();

});


/*
orchestrator.registerScenario("Scenario1: Create new course", async (s, t) => {
  const { alice, bob } = await s.players(
    { alice: conductorConfig, bob: conductorConfig },
    true
  );
  const course_addr = await alice.call(
    "course_dna",
    "courses",
    "create_course",
    {
      title: "course test 1"
      , timestamp: 123
    }
  );
  console.log(course_addr);
  t.ok(course_addr.Ok);

  await s.consistency();

  const courseResult = await bob.call("course_dna", "courses", "get_entry", {
    address: course_addr.Ok
  });
  const course = JSON.parse(courseResult.Ok.App[1]);
  console.log(course);
  t.deepEqual(course, {
    title: "course test 1",
    timestamp: 123,
    teacher_address: alice.instance("course_dna").agentAddress,
    modules: []
  });
  // Wait for all network activity to settle
  await s.consistency();
});
orchestrator.registerScenario("Scenario2: Update course title", async (s, t) => {
  const { alice, bob } = await s.players(
    { alice: conductorConfig, bob: conductorConfig },
    true
  );
  const course_addr = await alice.call(
    "course_dna",
    "courses",
    "create_course",
    {
      title: "new course test for update test"
      , timestamp: 123
    }
  );

  const course_update_addrss = await alice.call(
    "course_dna",
    "courses",
    "update_course",
    {
      title: "course title updated",
      course_address: course_addr.Ok,
      modules_addresses: [],
      timestamp: 123

    }
  );



  const courseResult = await bob.call("course_dna", "courses", "get_entry", {
    address: course_update_addrss.Ok
  });

  const course = JSON.parse(courseResult.Ok.App[1]);
  console.log(course);
  t.deepEqual(course, {
    title: "course title updated",
    timestamp: 123,
    teacher_address: alice.instance("course_dna").agentAddress,
    modules: []
  });



});
orchestrator.registerScenario("Scenario3: Delete course", async (s, t) => {
  const { alice, bob } = await s.players(
    { alice: conductorConfig, bob: conductorConfig },
    true
  );
  const course_addr = await alice.call(
    "course_dna",
    "courses",
    "create_course",
    {
      title: "new course test for delete scenario"
      , timestamp: 123
    }
  );
  await s.consistency();

  const delete_result = await alice.call(
    "course_dna",
    "courses",
    "delete_course",
    {
      course_address: course_addr.Ok,
    }
  );
  await s.consistency();

  t.ok(delete_result.Ok);

  const courseResult = await bob.call("course_dna", "courses", "get_entry", {
    address: delete_result.Ok
  });
  t.deepEqual(courseResult.Ok.Deletion.deleted_entry_address, course_addr.Ok);
  await s.consistency();

});

orchestrator.registerScenario("Scenario4: Create new Module for a Course", async (s, t) => {
  const { alice, bob } = await s.players(
    { alice: conductorConfig, bob: conductorConfig },
    true
  );
  const course_addr = await alice.call(
    "course_dna",
    "courses",
    "create_course",
    {
      title: "course for scenario 4"
      , timestamp: 123
    }
  );
  console.log(course_addr);
  t.ok(course_addr.Ok);

  await s.consistency();
  // Alice can create a module for course because she is the owner
  const new_module_addr = await alice.call("course_dna", "courses", "create_module", {
    title: "module 1 for course 1",
    course_address: course_addr.Ok,
    timestamp: 456
  });

  console.log(new_module_addr);
  t.ok(new_module_addr.Ok);
  await s.consistency();

  // Bob can not create a module for course, because he is not the owner of course
  const fail_add_module_addr = await bob.call("course_dna", "courses", "create_module", {
    title: "module 1 for course 1 by bob",
    course_address: course_addr.Ok,
    timestamp: 980
  });

  t.error(fail_add_module_addr.Ok);
  await s.consistency();

  const moduleResult = await alice.call("course_dna", "courses", "get_entry", {
    address: new_module_addr.Ok
  });
  const module = JSON.parse(moduleResult.Ok.App[1]);
  console.log(module);
  t.deepEqual(module, {
    title: "module 1 for course 1",
    course_address: course_addr.Ok,
    timestamp: 456
  });
  await s.consistency();
});


orchestrator.registerScenario("Scenario5: Get All My Courses", async (s, t) => {
  const { alice, bob } = await s.players(
    { alice: conductorConfig, bob: conductorConfig },
    true
  );
  const course_addr_1 = await alice.call(
    "course_dna",
    "courses",
    "create_course",
    {
      title: "course for scenario 5-1",
      timestamp: 123
    }
  );
  console.log(course_addr_1);
  t.ok(course_addr_1.Ok);

  await s.consistency();

  const course_addr_2 = await alice.call(
    "course_dna",
    "courses",
    "create_course",
    {
      title: "course for scenario 5-2",
      timestamp: 1234
    }
  );
  console.log(course_addr_2);
  t.ok(course_addr_2.Ok);

  await s.consistency();


  const all_courses_alice = await alice.call("course_dna", "courses", "get_my_courses", {
  });
  t.true(all_courses_alice.Ok[0] != null);
  t.true(all_courses_alice.Ok[1] != null);

  const all_courses_bob = await bob.call("course_dna", "courses", "get_my_courses", {
  });
  t.true(all_courses_bob.Ok[0] == null);

  await s.consistency();

});



orchestrator.registerScenario("Scenario6: Create new Content for a Module", async (s, t) => {
  const { alice, bob } = await s.players(
    { alice: conductorConfig, bob: conductorConfig },
    true
  );
  const course_addr = await alice.call(
    "course_dna",
    "courses",
    "create_course",
    {
      title: "course for scenario 5"
      , timestamp: 123
    }
  );
  console.log(course_addr);
  t.ok(course_addr.Ok);

  await s.consistency();
  const module_addr = await alice.call("course_dna", "courses", "create_module", {
    title: "module 1 for course 1",
    course_address: course_addr.Ok,
    timestamp: 456
  });

  console.log(module_addr);
  t.ok(module_addr.Ok);
  await s.consistency();

  const content_addr = await alice.call("course_dna", "courses", "create_content", {
    name: "content 1 for module 1",
    url: "https://youtube.com",
    descritpion: "Holochain Intro",
    module_address: module_addr.Ok,
    timestamp: 789
  });

  console.log(content_addr);
  t.ok(content_addr.Ok);
  await s.consistency();
});



orchestrator.registerScenario("Scenario7: Get all contents of a module", async (s, t) => {
  const { alice, bob } = await s.players(
    { alice: conductorConfig, bob: conductorConfig },
    true
  );
  const course_addr = await alice.call(
    "course_dna",
    "courses",
    "create_course",
    {
      title: "course for scenario 5"
      , timestamp: 123
    }
  );
  console.log(course_addr);
  t.ok(course_addr.Ok);

  await s.consistency();
  // Alice can create a module for course because she is the owner
  const module_addr = await alice.call("course_dna", "courses", "create_module", {
    title: "module 1 for course 1",
    course_address: course_addr.Ok,
    timestamp: 456
  });

  console.log(module_addr);
  t.ok(module_addr.Ok);
  await s.consistency();

  const content_addr_1 = await alice.call("course_dna", "courses", "create_content", {
    name: "content 1 for module 1",
    url: "https://youtube.com",
    descritpion: "Holochain Intro-Video",
    module_address: module_addr.Ok,
    timestamp: 7891
  });

  console.log(content_addr_1);
  t.ok(content_addr_1.Ok);
  await s.consistency();


  const content_addr_2 = await alice.call("course_dna", "courses", "create_content", {
    name: "content 2 for module 1",
    url: "https://soundclould.com",
    descritpion: "Holochain Intro-Sound",
    module_address: module_addr.Ok,
    timestamp: 7892
  });

  console.log(content_addr_2);
  t.ok(content_addr_2.Ok);
  await s.consistency();


  const all_contents_of_module_1 = await alice.call("course_dna", "courses", "get_contents", {
    module_address: module_addr.Ok
  });

  t.true(all_contents_of_module_1.Ok[0] != null);
  t.true(all_contents_of_module_1.Ok[1] != null);

  await s.consistency();
});



orchestrator.registerScenario("Scenario8: delete content from module", async (s, t) => {
  const { alice, bob } = await s.players(
    { alice: conductorConfig, bob: conductorConfig },
    true
  );
  const course_addr = await alice.call(
    "course_dna",
    "courses",
    "create_course",
    {
      title: "course for scenario 5"
      , timestamp: 123
    }
  );
  console.log(course_addr);
  t.ok(course_addr.Ok);

  await s.consistency();
  // Alice can create a module for course because she is the owner
  const module_addr = await alice.call("course_dna", "courses", "create_module", {
    title: "module 1 for course 1",
    course_address: course_addr.Ok,
    timestamp: 456
  });

  console.log(module_addr);
  t.ok(module_addr.Ok);
  await s.consistency();

  const content_addr_1 = await alice.call("course_dna", "courses", "create_content", {
    name: "content 1 for module 1",
    url: "https://youtube.com",
    descritpion: "Holochain Intro-Video",
    module_address: module_addr.Ok,
    timestamp: 7891
  });

  console.log(content_addr_1);
  t.ok(content_addr_1.Ok);
  await s.consistency();


  const content_addr_2 = await alice.call("course_dna", "courses", "create_content", {
    name: "content 2 for module 1",
    url: "https://soundclould.com",
    descritpion: "Holochain Intro-Sound",
    module_address: module_addr.Ok,
    timestamp: 7892
  });

  console.log(content_addr_2);
  t.ok(content_addr_2.Ok);
  await s.consistency();


  const all_contents_of_module_1 = await alice.call("course_dna", "courses", "get_contents", {
    module_address: module_addr.Ok
  });

  t.true(all_contents_of_module_1.Ok[0] != null);
  t.true(all_contents_of_module_1.Ok[1] != null);
  await s.consistency();

  const delete_content = await alice.call("course_dna", "courses", "delete_content", {
    content_address: content_addr_1.Ok
  });
  console.log("Hedayat_abedijoo_");
  console.log(delete_content);
  t.ok(delete_content.Ok);

  await s.consistency();

  // const all_contents_of_module_1_again = await alice.call("course_dna", "courses", "get_contents", {
  //   module_address: module_addr.Ok
  // });

  // t.true(all_contents_of_module_1.Ok[0] != null);
  // t.true(all_contents_of_module_1.Ok[1] == null);

  await s.consistency();
});
*/

orchestrator.registerScenario(
  "Scenario9: delete module from course, Testing bug scenario",
  async (s, t) => {
    const { alice, bob } = await s.players(
      { alice: conductorConfig, bob: conductorConfig },
      true
    );
    // Alice creates a test course
    const course_addr = await alice.call(
      "course_dna",
      "courses",
      "create_course",
      {
        title: "course for scenario 9: debugging purpose",
        timestamp: 123
      }
    );
    console.log("Hedayat_abedijoo_course_addr");
    console.log(course_addr);
    // verify that course was created succesfully
    t.ok(course_addr.Ok);

    await s.consistency();
    // Alice can create a module for course because she is the owner
    const module_addr = await alice.call(
      "course_dna",
      "courses",
      "create_module",
      {
        title: "module 1 for course 1",
        course_address: course_addr.Ok,
        timestamp: 456
      }
    );

    console.log(module_addr);
    // verify that module was created successfully
    t.ok(module_addr.Ok);
    await s.consistency();

    const delete_module = await alice.call(
      "course_dna",
      "courses",
      "delete_module",
      {
        module_address: module_addr.Ok
      }
    );
    console.log(delete_module);
    // verify that module was deleted successfully
    t.ok(delete_module.Ok);
    await s.consistency();

    // view all courses that belong to Alice
    const courseResult = await alice.call(
      "course_dna",
      "courses",
      "get_my_courses",
      {
        // address: course_addr.Ok
      }
    );
    console.log("Hedayat_abedijoo_getmycourse");
    // verify that listing all courses was successfull
    console.log(courseResult.Ok);
    // verify that course in the list matches the course Alice created in the beginning of the test
    //t.deepEqual(course_addr.Ok, courseResult.Ok[0]);
    await s.consistency();

    console.log("Hedayat_abedijoo_getentry");
    const course_again = await alice.call(
      "course_dna",
      "courses",
      "get_entry",
      {
        address: course_addr.Ok
      }
    );
    console.log(course_again.Ok);
    await s.consistency();

    // const content_addr_1 = await alice.call("course_dna", "courses", "create_content", {
    //   name: "content 1 for module 1",
    //   url: "https://youtube.com",
    //   descritpion: "Holochain Intro-Video",
    //   module_address: module_addr.Ok,
    //   timestamp: 7891
    // });

    // console.log(content_addr_1);
    // t.ok(content_addr_1.Ok);
    // await s.consistency();

    // const content_addr_2 = await alice.call("course_dna", "courses", "create_content", {
    //   name: "content 2 for module 1",
    //   url: "https://soundclould.com",
    //   descritpion: "Holochain Intro-Sound",
    //   module_address: module_addr.Ok,
    //   timestamp: 7892
    // });

    // console.log(content_addr_2);
    // t.ok(content_addr_2.Ok);
    // await s.consistency();

    // const all_contents_of_module_1 = await alice.call("course_dna", "courses", "get_contents", {
    //   module_address: module_addr.Ok
    // });

    // t.true(all_contents_of_module_1.Ok[0] != null);
    // t.true(all_contents_of_module_1.Ok[1] != null);
    // await s.consistency();

    // const delete_content = await alice.call("course_dna", "courses", "delete_content", {
    //   content_address: content_addr_1.Ok
    // });
    // console.log("Hedayat_abedijoo_");
    // console.log(delete_content);
    // t.ok(delete_content.Ok);

    // await s.consistency();

    // const all_contents_of_module_1_again = await alice.call("course_dna", "courses", "get_contents", {
    //   module_address: module_addr.Ok
    // });

    // t.true(all_contents_of_module_1.Ok[0] != null);
    // t.true(all_contents_of_module_1.Ok[1] == null);

    await s.consistency();
  }
);

orchestrator.run();
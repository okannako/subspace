(function() {var implementors = {};
implementors["polkadot_node_collation_generation"] = [{"text":"impl&lt;Context&gt; <a class=\"trait\" href=\"polkadot_overseer_gen/trait.Subsystem.html\" title=\"trait polkadot_overseer_gen::Subsystem\">Subsystem</a>&lt;Context, <a class=\"enum\" href=\"polkadot_node_subsystem_types/errors/enum.SubsystemError.html\" title=\"enum polkadot_node_subsystem_types::errors::SubsystemError\">SubsystemError</a>&gt; for <a class=\"struct\" href=\"polkadot_node_collation_generation/struct.CollationGenerationSubsystem.html\" title=\"struct polkadot_node_collation_generation::CollationGenerationSubsystem\">CollationGenerationSubsystem</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Context: <a class=\"trait\" href=\"polkadot_node_subsystem/trait.SubsystemContext.html\" title=\"trait polkadot_node_subsystem::SubsystemContext\">SubsystemContext</a>&lt;Message = <a class=\"enum\" href=\"polkadot_node_subsystem_types/messages/enum.CollationGenerationMessage.html\" title=\"enum polkadot_node_subsystem_types::messages::CollationGenerationMessage\">CollationGenerationMessage</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;Context: <a class=\"trait\" href=\"polkadot_overseer_gen/trait.SubsystemContext.html\" title=\"trait polkadot_overseer_gen::SubsystemContext\">SubsystemContext</a>&lt;Message = <a class=\"enum\" href=\"polkadot_node_subsystem_types/messages/enum.CollationGenerationMessage.html\" title=\"enum polkadot_node_subsystem_types::messages::CollationGenerationMessage\">CollationGenerationMessage</a>&gt;,&nbsp;</span>","synthetic":false,"types":["polkadot_node_collation_generation::CollationGenerationSubsystem"]}];
implementors["polkadot_node_core_chain_api"] = [{"text":"impl&lt;Client, Context&gt; <a class=\"trait\" href=\"polkadot_overseer_gen/trait.Subsystem.html\" title=\"trait polkadot_overseer_gen::Subsystem\">Subsystem</a>&lt;Context, <a class=\"enum\" href=\"polkadot_node_subsystem_types/errors/enum.SubsystemError.html\" title=\"enum polkadot_node_subsystem_types::errors::SubsystemError\">SubsystemError</a>&gt; for <a class=\"struct\" href=\"polkadot_node_core_chain_api/struct.ChainApiSubsystem.html\" title=\"struct polkadot_node_core_chain_api::ChainApiSubsystem\">ChainApiSubsystem</a>&lt;Client&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Client: HeaderBackend&lt;<a class=\"struct\" href=\"subspace_runtime_primitives/opaque/struct.Block.html\" title=\"struct subspace_runtime_primitives::opaque::Block\">Block</a>&gt; + AuxStore + 'static,<br>&nbsp;&nbsp;&nbsp;&nbsp;Context: <a class=\"trait\" href=\"polkadot_node_subsystem/trait.SubsystemContext.html\" title=\"trait polkadot_node_subsystem::SubsystemContext\">SubsystemContext</a>&lt;Message = <a class=\"enum\" href=\"polkadot_node_subsystem_types/messages/enum.ChainApiMessage.html\" title=\"enum polkadot_node_subsystem_types::messages::ChainApiMessage\">ChainApiMessage</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;Context: <a class=\"trait\" href=\"polkadot_overseer_gen/trait.SubsystemContext.html\" title=\"trait polkadot_overseer_gen::SubsystemContext\">SubsystemContext</a>&lt;Message = <a class=\"enum\" href=\"polkadot_node_subsystem_types/messages/enum.ChainApiMessage.html\" title=\"enum polkadot_node_subsystem_types::messages::ChainApiMessage\">ChainApiMessage</a>&gt;,&nbsp;</span>","synthetic":false,"types":["polkadot_node_core_chain_api::ChainApiSubsystem"]}];
implementors["polkadot_node_core_runtime_api"] = [{"text":"impl&lt;Client, Context&gt; <a class=\"trait\" href=\"polkadot_overseer_gen/trait.Subsystem.html\" title=\"trait polkadot_overseer_gen::Subsystem\">Subsystem</a>&lt;Context, <a class=\"enum\" href=\"polkadot_node_subsystem_types/errors/enum.SubsystemError.html\" title=\"enum polkadot_node_subsystem_types::errors::SubsystemError\">SubsystemError</a>&gt; for <a class=\"struct\" href=\"polkadot_node_core_runtime_api/struct.RuntimeApiSubsystem.html\" title=\"struct polkadot_node_core_runtime_api::RuntimeApiSubsystem\">RuntimeApiSubsystem</a>&lt;Client&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Client: ProvideRuntimeApi&lt;<a class=\"struct\" href=\"subspace_runtime_primitives/opaque/struct.Block.html\" title=\"struct subspace_runtime_primitives::opaque::Block\">Block</a>&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + 'static + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;Client::Api: <a class=\"trait\" href=\"sp_executor/trait.ExecutorApi.html\" title=\"trait sp_executor::ExecutorApi\">ExecutorApi</a>&lt;<a class=\"struct\" href=\"subspace_runtime_primitives/opaque/struct.Block.html\" title=\"struct subspace_runtime_primitives::opaque::Block\">Block</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;Context: <a class=\"trait\" href=\"polkadot_node_subsystem/trait.SubsystemContext.html\" title=\"trait polkadot_node_subsystem::SubsystemContext\">SubsystemContext</a>&lt;Message = <a class=\"enum\" href=\"polkadot_node_subsystem_types/messages/enum.RuntimeApiMessage.html\" title=\"enum polkadot_node_subsystem_types::messages::RuntimeApiMessage\">RuntimeApiMessage</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;Context: <a class=\"trait\" href=\"polkadot_overseer_gen/trait.SubsystemContext.html\" title=\"trait polkadot_overseer_gen::SubsystemContext\">SubsystemContext</a>&lt;Message = <a class=\"enum\" href=\"polkadot_node_subsystem_types/messages/enum.RuntimeApiMessage.html\" title=\"enum polkadot_node_subsystem_types::messages::RuntimeApiMessage\">RuntimeApiMessage</a>&gt;,&nbsp;</span>","synthetic":false,"types":["polkadot_node_core_runtime_api::RuntimeApiSubsystem"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()
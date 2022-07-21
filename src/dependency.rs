use serde::{Deserialize, Serialize};

/**
 * The states in the lifecycle of a service.
 */
#[derive(Serialize, Deserialize, Debug)]
pub enum State {
    /**
     * Object does not have a state (not a Lifecycle).
     */
    STATELESS,

    /**
     * Freshly created, probably being injected.
     */
    NEW,

    /**
     * Associated artifacts are installed.
     */
    INSTALLED,

    /**
     * The service has started, but hasn't report running yet.
     */
    STARTING,

    /**
     * Up and running, operating normally. This is the only state that should
     * ever take a significant amount of time to run.
     */
    RUNNING,

    /**
     * Service is in the process of shutting down.
     */
    STOPPING,

    /**
     * Not running. It may be possible for the enclosing framework to restart
     * it.
     */
    ERRORED,

    /**
     * Shut down, cannot be restarted. Generally the result of an unresolvable error.
     */
    BROKEN,
    /**
     * The service has done it's job and has no more to do. May be restarted
     * (for example, a monitoring task that will be restarted by a timer)
     */
    FINISHED,
}

initSidebarItems({"enum":[["ResponseError","An error returned a portal request caused by either the user cancelling the request or something else."]],"mod":[["desktop","Interact with the user's desktop such as taking a screenshot, setting a background or quering the user's location."],["documents","Interact with the documents store or transfer files across apps."],["flatpak","Spawn commands outside the sandbox or monitor if the running application has received an update & install it. # Examples"]],"struct":[["BasicResponse","The most basic response. Used when only the status of the request is what we receive as a response."],["HandleToken","A handle token is a DBus Object Path element, specified in the [`RequestProxy`] or [`SessionProxy`] object path following this format `/org/freedesktop/portal/desktop/request/SENDER/TOKEN` where sender is the caller's unique name and token is the HandleToken."],["NString","A Null terminated string."],["RequestProxy","The Request interface is shared by all portal interfaces. When a portal method is called, the reply includes a handle (i.e. object path) for a Request object, which will stay alive for the duration of the user interaction related to the method call."],["SessionProxy","The Session interface is shared by all portal interfaces that involve long lived sessions. When a method that creates a session is called, if successful, the reply will include a session handle (i.e. object path) for a Session object, which will stay alive for the duration of the session."],["WindowIdentifier","Most portals interact with the user by showing dialogs. These dialogs should generally be placed on top of the application window that triggered them. To arrange this, the compositor needs to know about the application window. Many portal requests expect a [`WindowIdentifier`] for this reason."]],"type":[["Response","A typical response returned by the `on_response` signal of a `RequestProxy`."]]});
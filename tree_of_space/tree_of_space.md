
<img width="562" alt="tree_of_space" src="https://user-images.githubusercontent.com/66478092/180999834-3a2990f5-7591-4486-bb14-8400f42d1421.png">

You need to do three operations on it 
1. lock(X, uid)
2. unlock(X, uid)
3. upgradeLock(x, uid)



where X the name of a node in the tree (that would be unique) and uid is the user who is performing the operation
Here are the denitions for the Operations:


Lock(X, uid)
Lock takes an exclusive access on the subtree rooted at X. It is formally dened like this: Once lock(X, uid) succeeds, then:
lock(A, anyUserId) should fail (returns false), where A is a descendent of X, lock(B, anyUserId) should fail (returns false), where X is a descendent of B
Lock operation cannot be performed on a node which is already locked i.e. lock(X, anyUserId) should fail (returns false).

Unlock(X, uid)
Unlock reverts what was done by the Lock operation. It can only be called on same node on which user uid had called a Lock on before. Returns true if it is successful.


UpgradeLock(X, uid)
It helps the user uid upgrade their lock to an ancestor node. It is only possible if the node X already has locked descendants and all of them are only locked by the same user uid. Upgrade should fail if there is any node which is descendant of X that is locked by a dierent user. Successful Upgrade will 'Lock' the node. UpgradeLock call shouldn't violate the consistency model that Lock/Unlock function requires.


Notes
1) The number of nodes in the tree N is very large. So, optimize the time complexity for the above algorithms
2) The below section contains the input format.

The rst line contains the number of Nodes in the tree (N).
The second line contains number of children per node (value m in m-ary Tree).
The third line contains number of queries (Q).
Next N lines contains the NodeName (string) in the m-Ary Tree.
Next Q lines contains queries which are in format: OperationType NodeName UserId

OperationType → 1 for Lock
2 for unlock
3 for upgradeLock
NodeName → Name of any node (unique) in m-Ary Tree.
UserId → Integer value representing a unique user.

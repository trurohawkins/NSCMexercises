void addToPool(ContactBook *contacts, Transaction *t) {
	Transaction *trans = calloc(1, sizeof(Transaction));
	memcpy(trans, t, sizeof(Transaction));
	addToList(&contacts->transactionPool, trans);
}

bool searchForTransaction(ContactBook *contacts, Transaction *t) {
	return cmpList(&contacts->transactionPool, t, compareTransactions);
}

bool removeFromTransactions(ContactBook *contacts, Transaction *t) {
	Transaction *removed = removeFromList(&contacts->transactionPool, t);
	if (removed) {
		return true;
	} else {
		return false;
	}
}

void printTransactionPool(ContactBook *contacts) {
	printList(&contacts->transactionPool, "Transaction Pool", printTransaction);
}

bool compareTransactions(Transaction *t1, Transaction *t2) {
	if (t1->timestamp == t2->timestamp) {
		if (t1->amount == t2->amount) {
			if (compareKeys(t1->sender, t2->sender) && compareKeys(t1->recipient, t2->recipient)) {
				return true;	
			}
		}
	}
	return false;
}


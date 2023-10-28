use std::arch::asm;

pub enum SyscallNumber {
	IoSetup                  = 0,
	IoDestroy                = 1,
	IoSubmit                 = 2,
	IoCancel                 = 3,
	IoGetevents              = 4,
	Setxattr                 = 5,
	Lsetxattr                = 6,
	Fsetxattr                = 7,
	Getxattr                 = 8,
	Lgetxattr                = 9,
	Fgetxattr                = 10,
	Listxattr                = 11,
	Llistxattr               = 12,
	Flistxattr               = 13,
	Removexattr              = 14,
	Lremovexattr             = 15,
	Fremovexattr             = 16,
	Getcwd                   = 17,
	LookupDcookie            = 18,
	Eventfd2                 = 19,
	EpollCreate1             = 20,
	EpollCtl                 = 21,
	EpollPwait               = 22,
	Dup                      = 23,
	Dup3                     = 24,
	Fcntl                    = 25,
	InotifyInit1             = 26,
	InotifyAddWatch          = 27,
	InotifyRmWatch           = 28,
	Ioctl                    = 29,
	IoprioSet                = 30,
	IoprioGet                = 31,
	Flock                    = 32,
	Mknodat                  = 33,
	Mkdirat                  = 34,
	Unlinkat                 = 35,
	Symlinkat                = 36,
	Linkat                   = 37,
	Renameat                 = 38,
	Umount2                  = 39,
	Mount                    = 40,
	PivotRoot                = 41,
	Nfsservctl               = 42,
	Statfs                   = 43,
	Fstatfs                  = 44,
	Truncate                 = 45,
	Ftruncate                = 46,
	Fallocate                = 47,
	Faccessat                = 48,
	Chdir                    = 49,
	Fchdir                   = 50,
	Chroot                   = 51,
	Fchmod                   = 52,
	Fchmodat                 = 53,
	Fchownat                 = 54,
	Fchown                   = 55,
	Openat                   = 56,
	Close                    = 57,
	Vhangup                  = 58,
	Pipe2                    = 59,
	Quotactl                 = 60,
	Getdents64               = 61,
	Lseek                    = 62,
	Read                     = 63,
	Write                    = 64,
	Readv                    = 65,
	Writev                   = 66,
	Pread64                  = 67,
	Pwrite64                 = 68,
	Preadv                   = 69,
	Pwritev                  = 70,
	Sendfile                 = 71,
	Pselect6                 = 72,
	Ppoll                    = 73,
	Signalfd4                = 74,
	Vmsplice                 = 75,
	Splice                   = 76,
	Tee                      = 77,
	Readlinkat               = 78,
	Fstatat                  = 79,
	Fstat                    = 80,
	Sync                     = 81,
	Fsync                    = 82,
	Fdatasync                = 83,
	SyncFileRange            = 84,
	TimerfdCreate            = 85,
	TimerfdSettime           = 86,
	TimerfdGettime           = 87,
	Utimensat                = 88,
	Acct                     = 89,
	Capget                   = 90,
	Capset                   = 91,
	Personality              = 92,
	Exit                     = 93,
	ExitGroup                = 94,
	Waitid                   = 95,
	SetTidAddress            = 96,
	Unshare                  = 97,
	Futex                    = 98,
	SetRobustList            = 99,
	GetRobustList            = 100,
	Nanosleep                = 101,
	Getitimer                = 102,
	Setitimer                = 103,
	KexecLoad                = 104,
	InitModule               = 105,
	DeleteModule             = 106,
	TimerCreate              = 107,
	TimerGettime             = 108,
	TimerGetoverrun          = 109,
	TimerSettime             = 110,
	TimerDelete              = 111,
	ClockSettime             = 112,
	ClockGettime             = 113,
	ClockGetres              = 114,
	ClockNanosleep           = 115,
	Syslog                   = 116,
	Ptrace                   = 117,
	SchedSetparam            = 118,
	SchedSetscheduler        = 119,
	SchedGetscheduler        = 120,
	SchedGetparam            = 121,
	SchedSetaffinity         = 122,
	SchedGetaffinity         = 123,
	SchedYield               = 124,
	SchedGetPriorityMax      = 125,
	SchedGetPriorityMin      = 126,
	SchedRrGetInterval       = 127,
	RestartSyscall           = 128,
	Kill                     = 129,
	Tkill                    = 130,
	Tgkill                   = 131,
	Sigaltstack              = 132,
	RtSigsuspend             = 133,
	RtSigaction              = 134,
	RtSigprocmask            = 135,
	RtSigpending             = 136,
	RtSigtimedwait           = 137,
	RtSigqueueinfo           = 138,
	RtSigreturn              = 139,
	Setpriority              = 140,
	Getpriority              = 141,
	Reboot                   = 142,
	Setregid                 = 143,
	Setgid                   = 144,
	Setreuid                 = 145,
	Setuid                   = 146,
	Setresuid                = 147,
	Getresuid                = 148,
	Setresgid                = 149,
	Getresgid                = 150,
	Setfsuid                 = 151,
	Setfsgid                 = 152,
	Times                    = 153,
	Setpgid                  = 154,
	Getpgid                  = 155,
	Getsid                   = 156,
	Setsid                   = 157,
	Getgroups                = 158,
	Setgroups                = 159,
	Uname                    = 160,
	Sethostname              = 161,
	Setdomainname            = 162,
	Getrlimit                = 163,
	Setrlimit                = 164,
	Getrusage                = 165,
	Umask                    = 166,
	Prctl                    = 167,
	Getcpu                   = 168,
	Gettimeofday             = 169,
	Settimeofday             = 170,
	Adjtimex                 = 171,
	Getpid                   = 172,
	Getppid                  = 173,
	Getuid                   = 174,
	Geteuid                  = 175,
	Getgid                   = 176,
	Getegid                  = 177,
	Gettid                   = 178,
	Sysinfo                  = 179,
	MqOpen                   = 180,
	MqUnlink                 = 181,
	MqTimedsend              = 182,
	MqTimedreceive           = 183,
	MqNotify                 = 184,
	MqGetsetattr             = 185,
	Msgget                   = 186,
	Msgctl                   = 187,
	Msgrcv                   = 188,
	Msgsnd                   = 189,
	Semget                   = 190,
	Semctl                   = 191,
	Semtimedop               = 192,
	Semop                    = 193,
	Shmget                   = 194,
	Shmctl                   = 195,
	Shmat                    = 196,
	Shmdt                    = 197,
	Socket                   = 198,
	Socketpair               = 199,
	Bind                     = 200,
	Listen                   = 201,
	Accept                   = 202,
	Connect                  = 203,
	Getsockname              = 204,
	Getpeername              = 205,
	Sendto                   = 206,
	Recvfrom                 = 207,
	Setsockopt               = 208,
	Getsockopt               = 209,
	Shutdown                 = 210,
	Sendmsg                  = 211,
	Recvmsg                  = 212,
	Readahead                = 213,
	Brk                      = 214,
	Munmap                   = 215,
	Mremap                   = 216,
	AddKey                   = 217,
	RequestKey               = 218,
	Keyctl                   = 219,
	Clone                    = 220,
	Execve                   = 221,
	Mmap                     = 222,
	Fadvise64                = 223,
	Swapon                   = 224,
	Swapoff                  = 225,
	Mprotect                 = 226,
	Msync                    = 227,
	Mlock                    = 228,
	Munlock                  = 229,
	Mlockall                 = 230,
	Munlockall               = 231,
	Mincore                  = 232,
	Madvise                  = 233,
	RemapFilePages           = 234,
	Mbind                    = 235,
	GetMempolicy             = 236,
	SetMempolicy             = 237,
	MigratePages             = 238,
	MovePages                = 239,
	RtTgsigqueueinfo         = 240,
	PerfEventOpen            = 241,
	Accept4                  = 242,
	Recvmmsg                 = 243,
	ArchSpecificSyscall      = 244,
	Wait4                    = 260,
	Prlimit64                = 261,
	FanotifyInit             = 262,
	FanotifyMark             = 263,
	NameToHandleAt           = 264,
	ROpenByHandleAt          = 265,
	ClockAdjtime             = 266,
	Syncfs                   = 267,
	Setns                    = 268,
	Sendmmsg                 = 269,
	ProcessVmReadv           = 270,
	ProcessVmWritev          = 271,
	Kcmp                     = 272,
	FinitModule              = 273,
	SchedSetattr             = 274,
	SchedGetattr             = 275,
	Renameat2                = 276,
	Seccomp                  = 277,
	Getrandom                = 278,
	MemfdCreate              = 279,
	Bpf                      = 280,
	Execveat                 = 281,
	Userfaultfd              = 282,
	Membarrier               = 283,
	Mlock2                   = 284,
	CopyFileRange            = 285,
	Preadv2                  = 286,
	Pwritev2                 = 287,
	PkeyMprotect             = 288,
	PkeyAlloc                = 289,
	PkeyFree                 = 290,
	Statx                    = 291,
	IoPgetevents             = 292,
	Rseq                     = 293,
	KexecFileLoad            = 294,
	ClockGettime64           = 403,
	ClockSettime64           = 404,
	ClockAdjtime64           = 405,
	ClockGetresTime64        = 406,
	ClockNanosleepTime64     = 407,
	TimerGettime64           = 408,
	TimerSettime64           = 409,
	TimerfdGettime64         = 410,
	TimerfdSettime64         = 411,
	UtimensatTime64          = 412,
	Pselect6Time64           = 413,
	PpollTime64              = 414,
	IoPgeteventsTime64       = 416,
	RecvmmsgTime64           = 417,
	MqTimedsendTime64        = 418,
	MqTimedreceiveTime64     = 419,
	SemtimedopTime64         = 420,
	RtSigtimedwaitTime64     = 421,
	FutexTime64              = 422,
	SchedRrGetIntervalTime64 = 423,
	PidfdSendSignal          = 424,
	IoUringSetup             = 425,
	IoUringEnter             = 426,
	IoUringRegister          = 427,
	OpenTree                 = 428,
	MoveMount                = 429,
	Fsopen                   = 430,
	Fsconfig                 = 431,
	Fsmount                  = 432,
	Fspick                   = 433,
	PidfdOpen                = 434,
	Clone3                   = 435,
	CloseRange               = 436,
	Openat2                  = 437,
	PidfdGetfd               = 438,
	Faccessat2               = 439,
	ProcessMadvise           = 440,
	EpollPwait2              = 441,
	MountSetattr             = 442,
	QuotactlFd               = 443,
	LandlockCreateRuleset    = 444,
	LandlockAddRule          = 445,
	LandlockRestrictSelf     = 446,
	MemfdSecret              = 447,
	ProcessMrelease          = 448
}

pub fn syscall0(num: isize) -> isize {
	let result;

	unsafe {
		asm!(
			"svc 0",
			lateout("x0") result,
			in("x8") num,
		);
	}

	result
}

pub fn syscall1(num: isize, arg1: isize) -> isize {
	let result;

	unsafe {
		asm!(
			"svc 0",
			lateout("x0") result,
			in("x8") num,
			in("x0") arg1
		);
	}

	result
}

pub fn syscall2(num: isize, arg1: isize, arg2: isize) -> isize {
	let result;

	unsafe {
		asm!(
			"svc 0",
			lateout("x0") result,
			in("x8") num,
			in("x0") arg1,
			in("x1") arg2
		);
	}

	result
}

pub fn syscall3(num: isize, arg1: isize, arg2: isize, arg3: isize) -> isize {
	let result;

	unsafe {
		asm!(
			"svc 0",
			lateout("x0") result,
			in("x8") num,
			in("x0") arg1,
			in("x1") arg2,
			in("x2") arg3
		);
	}

	result
}

pub fn syscall4(num: isize, arg1: isize, arg2: isize, arg3: isize, arg4: isize) -> isize {
	let result;

	unsafe {
		asm!(
			"svc 0",
			lateout("x0") result,
			in("x8") num,
			in("x0") arg1,
			in("x1") arg2,
			in("x2") arg3,
			in("x3") arg4
		);
	}

	result
}

pub fn syscall5(
	num: isize, arg1: isize, arg2: isize, arg3: isize, arg4: isize, arg5: isize
) -> isize {
	let result;

	unsafe {
		asm!(
			"svc 0",
			lateout("x0") result,
			in("x8") num,
			in("x0") arg1,
			in("x1") arg2,
			in("x2") arg3,
			in("x3") arg4,
			in("x4") arg5
		);
	}

	result
}

pub fn syscall6(
	num: isize, arg1: isize, arg2: isize, arg3: isize, arg4: isize, arg5: isize, arg6: isize
) -> isize {
	let result;

	unsafe {
		asm!(
			"svc 0",
			lateout("x0") result,
			in("x8") num,
			in("x0") arg1,
			in("x1") arg2,
			in("x2") arg3,
			in("x3") arg4,
			in("x4") arg5,
			in("x5") arg6
		);
	}

	result
}

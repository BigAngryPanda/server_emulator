typedef unsigned int        UINT;
typedef UINT                MTAPIRES;
typedef unsigned __int64    UINT64, *PUINT64;
typedef wchar_t WCHAR;
typedef const WCHAR *LPCWSTR, *PCWSTR;

typedef int                 INT;
typedef unsigned int        *PUINT;

typedef signed char         INT8, *PINT8;
typedef signed short        INT16, *PINT16;
typedef signed int          INT32, *PINT32;
typedef signed __int64      INT64, *PINT64;
typedef unsigned char       UINT8, *PUINT8;
typedef unsigned short      UINT16, *PUINT16;
typedef unsigned int        UINT32, *PUINT32;
typedef unsigned __int64    UINT64, *PUINT64;

typedef wchar_t MTAPISTR[260];

typedef unsigned long  DWORD;
typedef                DWORD COLORREF;
typedef unsigned short USHORT;
typedef unsigned char  UCHAR;

typedef int (__cdecl *MTSortFunctionPtr)(const void *left, const void *right);

typedef const void *LPCVOID;
typedef void *LPVOID;

typedef short SHORT;

typedef WCHAR *NWPSTR, *LPWSTR, *PWSTR;

typedef unsigned long ULONG;

typedef struct in6_addr {
    union {
        UCHAR       Byte[16];
        USHORT      Word[8];
    } u;
} IN6_ADDR;

#define NULL 0

enum EnMTAPIRetcode
  {
//--- successfully codes
   MT_RET_OK                    =0,       // ok
   MT_RET_OK_NONE               =1,       // ok, no data
//--- common errors
   MT_RET_ERROR                 =2,       // Common error
   MT_RET_ERR_PARAMS            =3,       // Invalid parameters
   MT_RET_ERR_DATA              =4,       // Invalid data
   MT_RET_ERR_DISK              =5,       // Disk error
   MT_RET_ERR_MEM               =6,       // Memory error
   MT_RET_ERR_NETWORK           =7,       // Network error
   MT_RET_ERR_PERMISSIONS       =8,       // Not enough permissions
   MT_RET_ERR_TIMEOUT           =9,       // Operation timeout
   MT_RET_ERR_CONNECTION        =10,      // No connection
   MT_RET_ERR_NOSERVICE         =11,      // Service is not available
   MT_RET_ERR_FREQUENT          =12,      // Too frequent requests
   MT_RET_ERR_NOTFOUND          =13,      // Not found
   MT_RET_ERR_PARTIAL           =14,      // Partial error
   MT_RET_ERR_SHUTDOWN          =15,      // Server shutdown in progress
   MT_RET_ERR_CANCEL            =16,      // Operation has been canceled
   MT_RET_ERR_DUPLICATE         =17,      // Duplicate data
//--- authentication retcodes
   MT_RET_AUTH_CLIENT_INVALID   =1000,    // Invalid terminal type
   MT_RET_AUTH_ACCOUNT_INVALID  =1001,    // Invalid account
   MT_RET_AUTH_ACCOUNT_DISABLED =1002,    // Account disabled
   MT_RET_AUTH_ADVANCED         =1003,    // Advanced authorization necessary
   MT_RET_AUTH_CERTIFICATE      =1004,    // Certificate required
   MT_RET_AUTH_CERTIFICATE_BAD  =1005,    // Invalid certificate
   MT_RET_AUTH_NOTCONFIRMED     =1006,    // Certificate is not confirmed
   MT_RET_AUTH_SERVER_INTERNAL  =1007,    // Attempt to connect to non-access server
   MT_RET_AUTH_SERVER_BAD       =1008,    // Server isn't authenticated 
   MT_RET_AUTH_UPDATE_ONLY      =1009,    // Only updates available
   MT_RET_AUTH_CLIENT_OLD       =1010,    // Client has old version
   MT_RET_AUTH_MANAGER_NOCONFIG =1011,    // Manager account doesn't have manager config
   MT_RET_AUTH_MANAGER_IPBLOCK  =1012,    // IP address unallowed for manager
   MT_RET_AUTH_GROUP_INVALID    =1013,    // Group is not initialized (server restart neccesary)
   MT_RET_AUTH_CA_DISABLED      =1014,    // Certificate generation disabled
   MT_RET_AUTH_INVALID_ID       =1015,    // Invalid or disabled server id [check server's id]
   MT_RET_AUTH_INVALID_IP       =1016,    // Unallowed address [check server's ip address]
   MT_RET_AUTH_INVALID_TYPE     =1017,    // Invalid server type [check server's id and type]
   MT_RET_AUTH_SERVER_BUSY      =1018,    // Server is busy
   MT_RET_AUTH_SERVER_CERT      =1019,    // Invalid server certificate
   MT_RET_AUTH_ACCOUNT_UNKNOWN  =1020,    // Unknown account
   MT_RET_AUTH_SERVER_OLD       =1021,    // Old server version
   MT_RET_AUTH_SERVER_LIMIT     =1022,    // Server cannot be connected due to license limitation
   MT_RET_AUTH_MOBILE_DISABLED  =1023,    // Mobile connection aren't allowed in server license
   MT_RET_AUTH_MANAGER_TYPE     =1024,    // Connection type is not permitted for manager
   MT_RET_AUTH_DEMO_DISABLED    =1025,    // Demo allocation disabled
   MT_RET_AUTH_RESET_PASSWORD   =1026,    // Master password must be changed
   MT_RET_AUTH_OTP_INVALID      =1027,    // Invalid One-time password
   MT_RET_AUTH_OTP_NEED_SECRET  =1028,    // Secret key for one-time password is required
   MT_RET_AUTH_MIGRATION_MT4    =1029,    // MetaTrader 4 password migration is necessary
   MT_RET_AUTH_MIGRATION_MT5    =1030,    // MetaTrader 5 password migration is necessary
   MT_RET_AUTH_INVALID_VERIFY   =1031,    // Invalid or expired confirmation code
   MT_RET_AUTH_VERIFY_BAD_EMAIL =1032,    // Email verification code could not be sent
   MT_RET_AUTH_VERIFY_BAD_PHONE =1033,    // Phone verification code could not be sent
   MT_RET_AUTH_API_DISABLED     =1034,    // Account API connection disabled
//--- config management retcodes
   MT_RET_CFG_LAST_ADMIN        =2000,    // Last admin config deleting
   MT_RET_CFG_LAST_ADMIN_GROUP  =2001,    // Last admin group cannot be deleted
   MT_RET_CFG_NOT_EMPTY         =2003,    // Accounts or trades in group/symbol
   MT_RET_CFG_INVALID_RANGE     =2004,    // Invalid accounts or trades ranges
   MT_RET_CFG_NOT_MANAGER_LOGIN =2005,    // Manager account is not from manager group
   MT_RET_CFG_BUILTIN           =2006,    // Built-in protected config
   MT_RET_CFG_DUPLICATE         =2007,    // Configuration duplicate
   MT_RET_CFG_LIMIT_REACHED     =2008,    // Configuration limit reached
   MT_RET_CFG_NO_ACCESS_TO_MAIN =2009,    // Invalid network configuration
   MT_RET_CFG_DEALER_ID_EXIST   =2010,    // Dealer with same ID exists
   MT_RET_CFG_BIND_ADDR_EXIST   =2011,    // Bind address already exists
   MT_RET_CFG_WORKING_TRADE     =2012,    // Attempt to delete working trade server
   MT_RET_CFG_GATEWAY_NAME_EXIST=2013,    // Gateway with same name exists
   MT_RET_CFG_SWITCH_TO_BACKUP  =2014,    // Server must be switched to backup mode
   MT_RET_CFG_NO_BACKUP_MODULE  =2015,    // Backup server module is absent
   MT_RET_CFG_NO_TRADE_MODULE   =2016,    // Trade server module is absent
   MT_RET_CFG_NO_HISTORY_MODULE =2017,    // History server module is absent
   MT_RET_CFG_ANOTHER_SWITCH    =2018,    // Another switching process in progress
   MT_RET_CFG_NO_LICENSE_FILE    =2019,   // License file is absent
   MT_RET_CFG_GATEWAY_LOGIN_EXIST=2020,   // Gateway with same login already exist
//--- client management retcodes
   MT_RET_USR_LAST_ADMIN        =3001,    // Last admin account deleting
   MT_RET_USR_LOGIN_EXHAUSTED   =3002,    // Logins range exhausted
   MT_RET_USR_LOGIN_PROHIBITED  =3003,    // Login reserved at another server
   MT_RET_USR_LOGIN_EXIST       =3004,    // Account already exists
   MT_RET_USR_SUICIDE           =3005,    // Attempt of self-deletion
   MT_RET_USR_INVALID_PASSWORD  =3006,    // Invalid account password
   MT_RET_USR_LIMIT_REACHED     =3007,    // Users limit reached
   MT_RET_USR_HAS_TRADES        =3008,    // Account has open trades
   MT_RET_USR_DIFFERENT_SERVERS =3009,    // Attempt to move account to different server
   MT_RET_USR_DIFFERENT_CURRENCY=3010,    // Attempt to move account to different currency group
   MT_RET_USR_IMPORT_BALANCE    =3011,    // Account balance import error
   MT_RET_USR_IMPORT_GROUP      =3012,    // Account import with invalid group
   MT_RET_USR_ACCOUNT_EXIST     =3013,    // Account already exist
   MT_RET_USR_IMPORT_ACCOUNT    =3014,    // Account trade data import error
   MT_RET_USR_IMPORT_POSITIONS  =3015,    // Account trade positions import error
   MT_RET_USR_IMPORT_ORDERS     =3016,    // Account open orders import error
   MT_RET_USR_IMPORT_DEALS      =3017,    // Account deals history import error
   MT_RET_USR_IMPORT_HISTORY    =3018,    // Account orders history import error
   MT_RET_USR_API_LIMIT_REACHED =3019,    // Users with API enabled limit reached
//--- trades management retcodes
   MT_RET_TRADE_LIMIT_REACHED   =4001,    // Orders or deals limit reached
   MT_RET_TRADE_ORDER_EXIST     =4002,    // Order already exists
   MT_RET_TRADE_ORDER_EXHAUSTED =4003,    // Orders range exhausted
   MT_RET_TRADE_DEAL_EXHAUSTED  =4004,    // Deals range exhausted
   MT_RET_TRADE_MAX_MONEY       =4005,    // Money limit reached
   MT_RET_TRADE_DEAL_EXIST      =4006,    // Deal already exists
   MT_RET_TRADE_ORDER_PROHIBITED=4007,    // Order ticket reserved at another server
   MT_RET_TRADE_DEAL_PROHIBITED =4008,    // Deal ticket reserved at another server
   MT_RET_TRADE_SPLIT_VOLUME    =4009,    // Volume of the new position is less than the minimum allowed
//--- report generation retcodes
   MT_RET_REPORT_SNAPSHOT       =5001,    // Base snapshot error
   MT_RET_REPORT_NOTSUPPORTED   =5002,    // Method doesn't support for this report
   MT_RET_REPORT_NODATA         =5003,    // No report data
   MT_RET_REPORT_TEMPLATE_BAD   =5004,    // Bad template
   MT_RET_REPORT_TEMPLATE_END   =5005,    // End of template (template success processed)
   MT_RET_REPORT_INVALID_ROW    =5006,    // Invalid row size
   MT_RET_REPORT_LIMIT_REPEAT   =5007,    // Tag repeat limit reached
   MT_RET_REPORT_LIMIT_REPORT   =5008,    // Report size limit reached
//--- price history reports retcodes
   MT_RET_HST_SYMBOL_NOTFOUND   =6001,    // Symbol not found, try to restart history server
//--- trade request retcodes
   MT_RET_REQUEST_INWAY                =10001, // Request on the way
   MT_RET_REQUEST_ACCEPTED             =10002, // Request accepted
   MT_RET_REQUEST_PROCESS              =10003, // Request processed
   MT_RET_REQUEST_REQUOTE              =10004, // Request Requoted
   MT_RET_REQUEST_PRICES               =10005, // Request Prices
   MT_RET_REQUEST_REJECT               =10006, // Request rejected
   MT_RET_REQUEST_CANCEL               =10007, // Request canceled
   MT_RET_REQUEST_PLACED               =10008, // Order from requestplaced
   MT_RET_REQUEST_DONE                 =10009, // Request executed
   MT_RET_REQUEST_DONE_PARTIAL         =10010, // Request executed partially
   MT_RET_REQUEST_ERROR                =10011, // Request common error
   MT_RET_REQUEST_TIMEOUT              =10012, // Request timeout
   MT_RET_REQUEST_INVALID              =10013, // Invalid request
   MT_RET_REQUEST_INVALID_VOLUME       =10014, // Invalid volume
   MT_RET_REQUEST_INVALID_PRICE        =10015, // Invalid price
   MT_RET_REQUEST_INVALID_STOPS        =10016, // Invalid stops or price
   MT_RET_REQUEST_TRADE_DISABLED       =10017, // Trade disabled
   MT_RET_REQUEST_MARKET_CLOSED        =10018, // Market closed
   MT_RET_REQUEST_NO_MONEY             =10019, // Not enough money
   MT_RET_REQUEST_PRICE_CHANGED        =10020, // Price changed
   MT_RET_REQUEST_PRICE_OFF            =10021, // No prices
   MT_RET_REQUEST_INVALID_EXP          =10022, // Invalid order expiration
   MT_RET_REQUEST_ORDER_CHANGED        =10023, // Order has been changed already
   MT_RET_REQUEST_TOO_MANY             =10024, // Too many trade requests
   MT_RET_REQUEST_NO_CHANGES           =10025, // Request doesn't contain changes
   MT_RET_REQUEST_AT_DISABLED_SERVER   =10026, // AutoTrading disabled by server
   MT_RET_REQUEST_AT_DISABLED_CLIENT   =10027, // AutoTrading disabled by client
   MT_RET_REQUEST_LOCKED               =10028, // Request locked by dealer
   MT_RET_REQUEST_FROZEN               =10029, // Order or position frozen
   MT_RET_REQUEST_INVALID_FILL         =10030, // Unsupported filling mode
   MT_RET_REQUEST_CONNECTION           =10031, // No connection
   MT_RET_REQUEST_ONLY_REAL            =10032, // Allowed for real accounts only
   MT_RET_REQUEST_LIMIT_ORDERS         =10033, // Orders limit reached
   MT_RET_REQUEST_LIMIT_VOLUME         =10034, // Volume limit reached
   MT_RET_REQUEST_INVALID_ORDER        =10035, // Invalid or prohibited order type
   MT_RET_REQUEST_POSITION_CLOSED      =10036, // Position doesn't exist
   MT_RET_REQUEST_EXECUTION_SKIPPED    =10037, // Execution doesn't belong to this server
   MT_RET_REQUEST_INVALID_CLOSE_VOLUME =10038, // Volume to be closed exceeds the position volume
   MT_RET_REQUEST_CLOSE_ORDER_EXIST    =10039, // Order to close this position already exists
   MT_RET_REQUEST_LIMIT_POSITIONS      =10040, // Positions limit reached
   MT_RET_REQUEST_REJECT_CANCEL        =10041, // Request rejected, order will be canceled
   MT_RET_REQUEST_LONG_ONLY            =10042, // Only long positions are allowed
   MT_RET_REQUEST_SHORT_ONLY           =10043, // Only short positions are allowed
   MT_RET_REQUEST_CLOSE_ONLY           =10044, // Only position closing is allowed
   MT_RET_REQUEST_PROHIBITED_BY_FIFO   =10045, // Position close prohibited by FIFO rule
   MT_RET_REQUEST_HEDGE_PROHIBITED     =10046, // Hedge is prohibited
//--- dealer retcodes
   MT_RET_REQUEST_RETURN               =11000, // Request returned in queue
   MT_RET_REQUEST_DONE_CANCEL          =11001, // Request partially filled, remainder has been canceled
   MT_RET_REQUEST_REQUOTE_RETURN       =11002, // Request requoted and returned in queue with new prices
//--- API retcodes               
   MT_RET_ERR_NOTIMPLEMENT             =12000, // Not implement yet
   MT_RET_ERR_NOTMAIN                  =12001, // Operation must be performed on main server
   MT_RET_ERR_NOTSUPPORTED             =12002, // Command doesn't supported
   MT_RET_ERR_DEADLOCK                 =12003, // Operation canceled due possible deadlock
   MT_RET_ERR_LOCKED                   =12004, // Operation on locked entity
//--- Messengers retcodes
   MT_RET_MESSENGER_INVALID_PHONE      =14000, // Invalid phone number
   MT_RET_MESSENGER_NOT_MOBILE         =14001, // Phone number isn't mobile
//--- Subscriptions retcodes
   MT_RET_SUBS_NOT_FOUND               =15000, // Subscription is not found
   MT_RET_SUBS_NOT_FOUND_CFG           =15001, // Subscription config is not found
   MT_RET_SUBS_NOT_FOUND_USER          =15002, // User for subscription is not found
   MT_RET_SUBS_DISABLED                =15003, // Subscription is disabled
   MT_RET_SUBS_PERMISSION_USER         =15004, // Subscription is not allowed for user
   MT_RET_SUBS_PERMISSION_SUBSCRIBE    =15005, // Subscribe is not allowed
   MT_RET_SUBS_PERMISSION_UNSUBSCRIBE  =15006, // Unsubscribe is not allowed
   MT_RET_SUBS_REAL_ONLY               =15007, // Subscriptions are available for real users only
  };

class IMTDatasetColumn
  {
public:
   //--- column data types
   enum EnType
     {
      //--- base types
      TYPE_INT8               =0,                     // Integer (8 bits)
      TYPE_UINT8              =1,                     // Unsigned Integer (8 bits)
      TYPE_INT16              =2,                     // Integer (16 bits)
      TYPE_UINT16             =3,                     // Unsigned Integer (16 bits)
      TYPE_INT32              =4,                     // Integer (32 bits)
      TYPE_UINT32             =5,                     // Unsigned Integer (32 bits)
      TYPE_INT64              =6,                     // Integer (64 bits)
      TYPE_UINT64             =7,                     // Unsigned Integer (64 bits)
      TYPE_DOUBLE             =8,                     // Double
      TYPE_MONEY              =9,                     // Money (Double)
      TYPE_STRING             =10,                    // Unicode String
      TYPE_DATE               =11,                    // Date (Int64)
      TYPE_TIME               =12,                    // Time (Int64)
      TYPE_DATETIME           =13,                    // Datetime (Int64)
      TYPE_TIME_MSC           =14,                    // Time in milliseconds (Int64)
      TYPE_DATETIME_MSC       =15,                    // Datetime in milliseconds (Int64)
      //--- prices
      TYPE_PRICE              =100,                   // Price (Double)
      TYPE_PRICES             =101,                   // Bid/Ask (Double[2])
      TYPE_PRICE_POSITION     =102,                   // Price for positions (Double)
      //--- volumes
      TYPE_VOLUME             =200,                   // Volume (UInt64)
      TYPE_VOLUME_ORDER       =201,                   // Initial Volume/Current Volume (UInt64[2])
      TYPE_VOLUME_EXT         =202,                   // Volume with extended accuracy (UInt64)
      TYPE_VOLUME_ORDER_EXT   =203,                   // Initial Volume/Current Volume with extended accuracy (UInt64[2])
      //--- positions
      TYPE_POSITION_TYPE      =300,                   // Position Type (UInt)
      //--- orders
      TYPE_ORDER_TYPE         =400,                   // Order Type (UInt)
      TYPE_ORDER_TYPE_TIME    =401,                   // Order Type by Time (UInt)
      TYPE_ORDER_TYPE_REASON  =402,                   // Order Type by Reason (UInt)
      TYPE_ORDER_STATUS       =403,                   // Order Status (UInt)
      TYPE_ORDER_FILLING      =404,                   // Order Filling (UInt)
      //--- deals
      TYPE_DEAL_ACTION        =500,                   // Deal Action (UInt)
      TYPE_DEAL_ENTRY         =501,                   // Deal Entry (UInt)
      //--- accounts
      TYPE_USER_LOGIN         =600,                   // Account Login (UInt64)
      TYPE_USER_LEVERAGE      =601,                   // Account Leverage (UInt)
      //--- clients
      TYPE_CLIENT_ID          =700,                   // Client Id (UInt64)
      //--- enumeration borders
      TYPE_FIRST              =TYPE_INT8,
      TYPE_LAST               =TYPE_CLIENT_ID
     };
   //--- column flags
   enum EnFlags
     {
      FLAG_NONE               =0x00000000,            // none flags
      FLAG_PRIMARY            =0x00000001,            // primary integer column
      FLAG_HIDDEN_VIEW        =0x00000002,            // hidden in grid view
      FLAG_HIDDEN_SAVE        =0x00000004,            // hidden in saved file
      FLAG_HIDDEN             =FLAG_HIDDEN_VIEW|FLAG_HIDDEN_SAVE, // hidden in grid and file
      FLAG_LEFT               =0x00000008,            // force align left
      FLAG_RIGHT              =0x00000010,            // force align right
      FLAG_CENTER             =FLAG_LEFT|FLAG_RIGHT,  // force align center
      FLAG_SORT_DEFAULT       =0x00000100,            // deafult sort column
      //--- enumeration borders
      FLAG_ALL                =FLAG_PRIMARY|FLAG_HIDDEN|FLAG_CENTER|FLAG_SORT_DEFAULT
     };
   //--- color
   enum EnColumnColor
     {
      COLUMN_COLOR_AUTO       =0xFFFFFFFF,            // auto color
     };
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTDatasetColumn *column)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- name
   virtual LPCWSTR   Name(void) const=0;
   virtual MTAPIRES  Name(LPCWSTR name)=0;
   //--- column id
   virtual UINT      ColumnID(void) const=0;
   virtual MTAPIRES  ColumnID(const UINT column_id)=0;
   //--- type IMTDatasetColumn::EnType
   virtual UINT      Type(void) const=0;
   virtual MTAPIRES  Type(const UINT type)=0;
   //--- column relative width
   virtual UINT      Width(void) const=0;
   virtual MTAPIRES  Width(const UINT width)=0;
   //--- column max width in pixels
   virtual UINT      WidthMax(void) const=0;
   virtual MTAPIRES  WidthMax(const UINT width_max)=0;
   //--- default digits
   virtual UINT      Digits(void) const=0;
   virtual MTAPIRES  Digits(const UINT digits)=0;
   //--- column digits reference
   virtual UINT      DigitsColumn(void) const=0;
   virtual MTAPIRES  DigitsColumn(const UINT column_id)=0;
   //--- flags IMTDatasetColumn::EnFlags
   virtual UINT64    Flags(void) const=0;
   virtual MTAPIRES  Flags(const UINT64 flags)=0;
   //--- offset
   virtual UINT      Offset(void) const=0;
   virtual MTAPIRES  Offset(const UINT offset)=0;
   //--- size
   virtual UINT      Size(void) const=0;
   virtual MTAPIRES  Size(const UINT size)=0;
   //--- chart color
   virtual UINT      Color(void) const=0;
   virtual MTAPIRES  Color(const UINT color)=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTDatasetColumn(void) {}
  };
//+------------------------------------------------------------------+
//| Dataset summary interface                                        |
//+------------------------------------------------------------------+
class IMTDatasetSummary
  {
public:
   //--- summary data types
   enum EnType
     {
      //--- base types
      TYPE_INT                =0,                     // Integer (64 bits)
      TYPE_UINT               =1,                     // Unsigned Integer (64 bits)
      TYPE_DOUBLE             =2,                     // Double
      TYPE_MONEY              =3,                     // Money
      TYPE_STRING             =4,                     // Unicode String
      TYPE_DATE               =5,                     // Date
      TYPE_TIME               =6,                     // Time
      TYPE_DATETIME           =7,                     // Datetime
      //--- prices
      TYPE_PRICE              =100,                   // Price
      TYPE_PRICES             =101,                   // Bid/Ask
      //--- volumes
      TYPE_VOLUME             =200,                   // Volume
      TYPE_VOLUME_ORDER       =201,                   // Initial Volume/Current Volume (UInt64[2])
      TYPE_VOLUME_EXT         =202,                   // Volume with extended accuracy (UInt64)
      TYPE_VOLUME_ORDER_EXT   =203,                   // Initial Volume/Current Volume with extended accuracy (UInt64[2])
      //--- enumeration borders
      TYPE_FIRST              =TYPE_INT,
      TYPE_LAST               =TYPE_VOLUME_ORDER_EXT
     };
   //--- summary flags
   enum EnFlags
     {
      FLAG_NONE               =0x0,                   // none flag
     };
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTDatasetSummary *summary)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- column id
   virtual UINT      ColumnID(void) const=0;
   virtual MTAPIRES  ColumnID(UINT column_id)=0;
   //--- line index
   virtual UINT      Line(void) const=0;
   virtual MTAPIRES  Line(UINT line)=0;
   //--- column for merging (merging from ColumnID to MergeColumn)
   virtual UINT      MergeColumn(void) const=0;
   virtual MTAPIRES  MergeColumn(UINT column_id)=0;
   //--- text color
   virtual UINT      Color(void) const=0;
   virtual MTAPIRES  Color(UINT color)=0;
   //--- flags IMTDatasetSummary::EnFlags
   virtual UINT64    Flags(void) const=0;
   virtual MTAPIRES  Flags(UINT64 flags)=0;
   //--- type IMTDatasetSummary::EnType
   virtual UINT      Type(void) const=0;
   //--- digits
   virtual UINT      Digits(void) const=0;
   virtual MTAPIRES  Digits(UINT digits)=0;
   //--- integer
   virtual INT64     ValueInt(void) const=0;
   virtual MTAPIRES  ValueInt(INT64 value)=0;
   //--- unsigned integer
   virtual UINT64    ValueUInt(void) const=0;
   virtual MTAPIRES  ValueUInt(UINT64 value)=0;
   //--- double
   virtual double    ValueDouble(void) const=0;
   virtual MTAPIRES  ValueDouble(double value)=0;
   //--- money
   virtual double    ValueMoney(void) const=0;
   virtual MTAPIRES  ValueMoney(double value)=0;
   //--- string
   virtual LPCWSTR   ValueString(void) const=0;
   virtual MTAPIRES  ValueString(LPCWSTR value)=0;
   //--- date
   virtual INT64     ValueDate(void) const=0;
   virtual MTAPIRES  ValueDate(INT64 value)=0;
   //--- time
   virtual INT64     ValueTime(void) const=0;
   virtual MTAPIRES  ValueTime(INT64 value)=0;
   //--- datetime
   virtual INT64     ValueDateTime(void) const=0;
   virtual MTAPIRES  ValueDateTime(INT64 value)=0;
   //--- price
   virtual double    ValuePrice(void) const=0;
   virtual MTAPIRES  ValuePrice(double value)=0;
   //--- prices
   virtual double    ValuePricesBid(void) const=0;
   virtual double    ValuePricesAsk(void) const=0;
   virtual MTAPIRES  ValuePrices(double value_bid,double value_ask)=0;
   //--- volume
   virtual UINT64    ValueVolume(void) const=0;
   virtual MTAPIRES  ValueVolume(UINT64 value)=0;
   //--- order volume
   virtual UINT64    ValueVolumeInitial(void) const=0;
   virtual UINT64    ValueVolumeCurrent(void) const=0;
   virtual MTAPIRES  ValueVolume(UINT64 value_initial,UINT64 value_current)=0;
   //--- volume with extended accuracy
   virtual UINT64    ValueVolumeExt(void) const=0;
   virtual MTAPIRES  ValueVolumeExt(UINT64 value)=0;
   //--- order volume with extended accuracy
   virtual UINT64    ValueVolumeExtInitial(void) const=0;
   virtual UINT64    ValueVolumeExtCurrent(void) const=0;
   virtual MTAPIRES  ValueVolumeExt(UINT64 value_initial,UINT64 value_current)=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTDatasetSummary(void) {}
  };

class IMTDataset
  {
public:
   //--- flags
   enum EnDataSetFlags
     {
      DATASET_FLAG_NONE     =0x0,    // no flags
     };

public:
   //--- common methods
   virtual MTAPIRES  Assign(const IMTDataset *data)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- flags
   virtual UINT64    Flags(void) const=0;
   virtual MTAPIRES  Flags(UINT64 flags)=0;
   //--- reserved properties methods
   virtual MTAPIRES  Reserved1(void)=0;
   virtual MTAPIRES  Reserved2(void)=0;
   virtual MTAPIRES  Reserved3(void)=0;
   virtual MTAPIRES  Reserved4(void)=0;
   //--- data set column management
   virtual IMTDatasetColumn* ColumnCreate(void)=0;
   virtual MTAPIRES  ColumnClear(void)=0;
   virtual MTAPIRES  ColumnAdd(const IMTDatasetColumn *column)=0;
   virtual MTAPIRES  ColumnDelete(UINT pos)=0;
   virtual UINT      ColumnTotal(void) const=0;
   virtual UINT      ColumnSize(void) const=0;
   virtual MTAPIRES  ColumnNext(UINT pos,IMTDatasetColumn *column)=0;
   virtual MTAPIRES  ColumnReserved1(void)=0;
   virtual MTAPIRES  ColumnReserved2(void)=0;
   //--- data set row management
   virtual MTAPIRES  RowClear(void)=0;
   virtual MTAPIRES  RowWrite(const void *data,UINT size)=0;
   virtual UINT      RowTotal(void) const=0;
   virtual MTAPIRES  RowRead(UINT pos,void *data,UINT size) const=0;
   virtual MTAPIRES  RowReserved2(void)=0;
   //--- data set summary management
   virtual IMTDatasetSummary* SummaryCreate(void)=0;
   virtual MTAPIRES  SummaryClear(void)=0;
   virtual MTAPIRES  SummaryAdd(const IMTDatasetSummary *summary)=0;
   virtual MTAPIRES  SummaryDelete(UINT pos)=0;
   virtual MTAPIRES  SummaryNext(UINT pos,IMTDatasetSummary *summary)=0;
   virtual UINT      SummaryTotal(void) const=0;
   virtual MTAPIRES  SummaryReserved1(void)=0;
   virtual MTAPIRES  SummaryReserved2(void)=0;
   //--- life control
   virtual void      Release(void)=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTDataset(void) {}
  };

class IMTDatasetField
  {
public:
   //--- filed type
   enum EnFieldType
     {
      TYPE_NONE                              =0,            // none
      TYPE_INT                               =1,            // integer
      TYPE_UINT                              =2,            // unsigned integer
      TYPE_DOUBLE                            =3,            // double
      TYPE_STRING                            =4,            // string
      //--- enumeration borders
      TYPE_FIRST                             =TYPE_NONE,
      TYPE_LAST                              =TYPE_STRING
     };
   //--- Request fields id enumeration
   enum EnFieldId
     {
      //--- User fields enumeration
      FIELD_USER_LOGIN                       =1,            // UINT64      , login
      FIELD_USER_GROUP                       =2,            // wchar_t[64] , group
      FIELD_USER_CERT_SERIAL_NUMBER          =3,            // UINT64      , certificate serial number
      FIELD_USER_RIGHTS                      =4,            // UINT64      , EnUsersRights
      FIELD_USER_REGISTRATION                =5,            // INT64       , registration datetime (filled by MT5)
      FIELD_USER_LAST_ACCESS                 =6,            // INT64       , last access datetime (filled by MT5)
      FIELD_USER_NAME                        =7,            // wchar_t[256], name
      FIELD_USER_COMPANY                     =8,            // wchar_t[32] , company
      FIELD_USER_ACCOUNT                     =9,            // wchar_t[64] , external system account (exchange, ECN, etc)
      FIELD_USER_COUNTRY                     =10,           // wchar_t[32] , country
      FIELD_USER_LANGUAGE                    =11,           // UINT        , client language (WinAPI LANGID)
      FIELD_USER_CITY                        =12,           // wchar_t[32] , city
      FIELD_USER_STATE                       =13,           // wchar_t[32] , state
      FIELD_USER_ZIP_CODE                    =14,           // wchar_t[16] , ZIP code
      FIELD_USER_ADDRESS                     =15,           // wchar_t[128], address
      FIELD_USER_PHONE                       =16,           // wchar_t[32] , phone
      FIELD_USER_EMAIL                       =17,           // wchar_t[64] , email
      FIELD_USER_ID                          =18,           // wchar_t[32] , additional ID
      FIELD_USER_STATUS                      =19,           // wchar_t[16] , additional status
      FIELD_USER_COMMENT                     =20,           // wchar_t[64] , comment
      FIELD_USER_COLOR                       =21,           // UINT        , color (WinAPI COLORREF)
      FIELD_USER_PHONE_PASSWORD              =22,           // wchar_t[32] , phone password
      FIELD_USER_LEVERAGE                    =23,           // UINT        , leverage
      FIELD_USER_AGENT                       =24,           // UINT64      , agent account
      FIELD_USER_BALANCE                     =25,           // double      , balance
      FIELD_USER_CREDIT                      =26,           // double      , credit
      FIELD_USER_INTEREST_RATE               =27,           // double      , accumulated interest rate
      FIELD_USER_COMMISSION_DAILY            =28,           // double      , accumulated daily commissions
      FIELD_USER_COMMISSION_MONTHLY          =29,           // double      , accumulated monthly commissions
      FIELD_USER_COMMISSION_AGENT_DAILY      =30,           // double      , accumulated daily agent commissions
      FIELD_USER_COMMISSION_AGENT_MONTHLY    =31,           // double      , accumulated monthly agent commissions
      FIELD_USER_BALANCE_PREV_DAY            =32,           // double      , previous day balance state
      FIELD_USER_BALANCE_PREV_MONTH          =33,           // double      , previous month balance state
      FIELD_USER_EQUITY_PREV_DAY             =34,           // double      , previous day equity state
      FIELD_USER_EQUITY_PREV_MONTH           =35,           // double      , previous month equity state
      FIELD_USER_LAST_PASS_CHANGE            =36,           // INT64       , last password change datetime (filled by MT5)
      FIELD_USER_MQID                        =37,           // UINT        , client's MetaQuotes ID
      FIELD_USER_LEAD_CAMPAIGN               =38,           // wchar_t[32] , lead campaign
      FIELD_USER_LEAD_SOURCE                 =39,           // wchar_t[32] , lead source
      FIELD_USER_CLIENT_ID                   =40,           // UINT64      , client id
      FIELD_USER_FIRST_NAME                  =41,           // wchar_t[128], first name
      FIELD_USER_LAST_NAME                   =42,           // wchar_t[64] , last name
      FIELD_USER_MIDDLE_NAME                 =43,           // wchar_t[64] , middle name
      //--- User fields enumeration borders
      FIELD_USER_FIRST                       =FIELD_USER_LOGIN,
      FIELD_USER_LAST                        =FIELD_USER_MIDDLE_NAME,
      //--- Client fields enumeration
      FIELD_CLIENT_ID                        =1001,         // UINT64      , client id
      FIELD_CLIENT_CREATED_TIME              =1002,         // INT64       , creation date and time
      FIELD_CLIENT_CREATED_BY                =1003,         // UINT64      , login of manager who created the client
      FIELD_CLIENT_MODIFIED_TIME             =1004,         // INT64       , modification date
      FIELD_CLIENT_MODIFIED_BY               =1005,         // UINT64      , login of manager who modified the client
      FIELD_CLIENT_TYPE                      =1006,         // UINT        , client type (EnClientType)
      FIELD_CLIENT_STATUS                    =1007,         // UINT        , client status (EnClientStatus)
      FIELD_CLIENT_ASSIGNED_MANAGER          =1008,         // UINT64      , assigned manager login
      FIELD_CLIENT_COMMENT                   =1009,         // wchar_t[128], comment
      FIELD_CLIENT_COMPLIANCE_APPROVED_BY    =1010,         // UINT64      , login of manager who approved the client
      FIELD_CLIENT_COMPLIANCE_CLIENT_CATEGORY=1011,         // wchar_t[64] , customer classification according to the rules of the local regulator
      FIELD_CLIENT_COMPLIANCE_TIME_APPROVAL  =1012,         // INT64       , approval date and time
      FIELD_CLIENT_COMPLIANCE_TIME_TERMINATION=1013,        // INT64       , agreement termination date and time
      FIELD_CLIENT_LEAD_CAMPAIGN             =1014,         // wchar_t[64] , lead campaign
      FIELD_CLIENT_LEAD_SOURCE               =1015,         // wchar_t[64] , lead source
      FIELD_CLIENT_INTRODUCER                =1016,         // wchar_t[32] , IB or Referal
      FIELD_CLIENT_PERSON_TITLE              =1017,         // wchar_t[16] , title
      FIELD_CLIENT_PERSON_NAME               =1018,         // wchar_t[32] , first name
      FIELD_CLIENT_PERSON_MIDDLE_NAME        =1019,         // wchar_t[32] , middle name
      FIELD_CLIENT_PERSON_LAST_NAME          =1020,         // wchar_t[32] , second name
      FIELD_CLIENT_PERSON_BIRTH_DATE         =1021,         // INT64       , date of birth (file time, number of 100-nanosecond since 12:00 A.M. January 1, 1601 UTC)
      FIELD_CLIENT_PERSON_CITIZENSHIP        =1022,         // wchar_t[64] , citizenship
      FIELD_CLIENT_PERSON_GENDER             =1023,         // UINT        , gender (EnGender)
      FIELD_CLIENT_PERSON_TAX_ID             =1024,         // wchar_t[64] , tax id
      FIELD_CLIENT_PERSON_DOCUMENT_TYPE      =1025,         // wchar_t[16] , document type
      FIELD_CLIENT_PERSON_DOCUMENT_NUMBER    =1026,         // wchar_t[32] , document number
      FIELD_CLIENT_PERSON_DOCUMENT_DATE      =1027,         // INT64       , document issue or expiration date
      FIELD_CLIENT_PERSON_DOCUMENT_EXTRA     =1028,         // wchar_t[64] , document extra data
      FIELD_CLIENT_PERSON_EMPLOYMENT         =1029,         // UINT        , employment status (EnEmployment)
      FIELD_CLIENT_PERSON_INDUSTRY           =1030,         // UINT        , employment industry (EnEmploymentIndustry)
      FIELD_CLIENT_PERSON_EDUCATION          =1031,         // UINT        , education level (EnEducationLevel)
      FIELD_CLIENT_PERSON_WEALTH_SOURCE      =1032,         // UINT        , wealth source (EnWealthSource)
      FIELD_CLIENT_PERSON_ANNUAL_INCOME      =1033,         // double      , annual income, USD
      FIELD_CLIENT_PERSON_NET_WORTH          =1034,         // double      , net worth, USD
      FIELD_CLIENT_PERSON_ANNUAL_DEPOSIT     =1035,         // double      , annual deposit, USD
      FIELD_CLIENT_COMPANY_NAME              =1036,         // wchar_t[64] , company name
      FIELD_CLIENT_COMPANY_REG_NUMBER        =1037,         // wchar_t[64] , company registration number
      FIELD_CLIENT_COMPANY_REG_DATE          =1038,         // INT64       , company registration date
      FIELD_CLIENT_COMPANY_REG_AUTHORITY     =1039,         // wchar_t[64] , company registration authority
      FIELD_CLIENT_COMPANY_VAT               =1040,         // wchar_t[64] , VAT registration number
      FIELD_CLIENT_COMPANY_LEI               =1041,         // wchar_t[64] , EI number for EMIR reports
      FIELD_CLIENT_COMPANY_LICENSE_NUMBER    =1042,         // wchar_t[64] , license number
      FIELD_CLIENT_COMPANY_LICENSE_AUTHORITY =1043,         // wchar_t[64] , license issuer authority
      FIELD_CLIENT_COMPANY_COUNTRY           =1044,         // wchar_t[64] , company registration country
      FIELD_CLIENT_COMPANY_ADDRESS           =1045,         // wchar_t[64] , company registration address
      FIELD_CLIENT_COMPANY_WEBSITE           =1046,         // wchar_t[64] , company website
      FIELD_CLIENT_CONTACT_PREFERRED         =1047,         // UINT        , preferred communication type
      FIELD_CLIENT_CONTACT_LANGUAGE          =1048,         // wchar_t[64] , communication language
      FIELD_CLIENT_CONTACT_EMAIL             =1049,         // wchar_t[64] , email (or multiple comma-separated emails)
      FIELD_CLIENT_CONTACT_PHONE             =1050,         // wchar_t[128], phone numbers (or multiple comma-separated phone numbers)
      FIELD_CLIENT_CONTACT_MESSENGERS        =1051,         // wchar_t[128], messengers in format: "skype:username, qq:5454535454"
      FIELD_CLIENT_CONTACT_SOCIALNETWORKS    =1052,         // wchar_t[128], social networks ids: "fb:user_id, vk:user_id"
      FIELD_CLIENT_CONTACT_LAST_DATE         =1053,         // INT64       , date and time of the last contact
      FIELD_CLIENT_ADDRESS_COUNTRY           =1054,         // wchar_t[64] , address - country
      FIELD_CLIENT_ADDRESS_POSTCODE          =1055,         // wchar_t[16] , address - zip code
      FIELD_CLIENT_ADDRESS_STREET            =1056,         // wchar_t[128], address - street address
      FIELD_CLIENT_ADDRESS_STATE             =1057,         // wchar_t[64] , address - state, province or district
      FIELD_CLIENT_ADDRESS_CITY              =1058,         // wchar_t[64] , address - city or town
      FIELD_CLIENT_EXPERIENCE_FX             =1059,         // UINT        , Forex trading experience (EnTradingExperience)
      FIELD_CLIENT_EXPERIENCE_CFD            =1060,         // UINT        , CFD trading experience (EnTradingExperience)
      FIELD_CLIENT_EXPERIENCE_FUTURES        =1061,         // UINT        , futures trading experience (EnTradingExperience)
      FIELD_CLIENT_EXPERIENCE_STOCKS         =1062,         // UINT        , stocks trading experience (EnTradingExperience)
      //--- Client fields enumeration borders
      FIELD_CLIENT_FIRST                     =FIELD_CLIENT_ID,
      FIELD_CLIENT_LAST                      =FIELD_CLIENT_EXPERIENCE_STOCKS,
      //--- Deal fields enumeration
      FIELD_DEAL_DEAL                        =2001,         // UINT64      , deal ticket
      FIELD_DEAL_EXTERNAL_ID                 =2002,         // wchar_t[32] , deal ticket in external system (exchange, ECN, etc)
      FIELD_DEAL_LOGIN                       =2003,         // UINT64      , client login
      FIELD_DEAL_DEALER                      =2004,         // UINT64      , processed dealer login (0-means auto)
      FIELD_DEAL_ORDER                       =2005,         // UINT64      , deal order ticket
      FIELD_DEAL_ACTION                      =2006,         // UINT        , IMTDeal::EnDealAction
      FIELD_DEAL_ENTRY                       =2007,         // UINT        , IMTDeal::EnDealEntry
      FIELD_DEAL_DIGITS                      =2008,         // UINT        , price digits
      FIELD_DEAL_DIGITS_CURRENCY             =2009,         // UINT        , currency digits
      FIELD_DEAL_CONTRACT_SIZE               =2010,         // double      , symbol contract size
      FIELD_DEAL_TIME                        =2011,         // INT64       , deal creation datetime in seconds
      FIELD_DEAL_SYMBOL                      =2012,         // wchar_t[32] , deal symbol
      FIELD_DEAL_PRICE                       =2013,         // double      , deal price
      FIELD_DEAL_VOLUME_EXT                  =2014,         // UINT64      , deal volume with extended accuracy
      FIELD_DEAL_PROFIT                      =2015,         // double      , deal profit
      FIELD_DEAL_STORAGE                     =2016,         // double      , deal collected swaps
      FIELD_DEAL_COMMISSION                  =2017,         // double      , deal commission
      FIELD_DEAL_RATE_PROFIT                 =2018,         // double      , profit conversion rate (from symbol profit currency to deposit currency)
      FIELD_DEAL_RATE_MARGIN                 =2019,         // double      , margin conversion rate (from symbol margin currency to deposit currency)
      FIELD_DEAL_EXPERT_ID                   =2020,         // UINT64      , expert id (filled by expert advisor)
      FIELD_DEAL_POSITION_ID                 =2021,         // UINT64      , position id
      FIELD_DEAL_COMMENT                     =2022,         // wchar_t[32] , deal comment
      FIELD_DEAL_PROFIT_RAW                  =2023,         // double      , deal profit in symbol's profit currency
      FIELD_DEAL_PRICE_POSITION              =2024,         // double      , closed position  price
      FIELD_DEAL_VOLUME_CLOSED_EXT           =2025,         // UINT64      , closed volume with extended accuracy
      FIELD_DEAL_TICK_VALUE                  =2026,         // double      , tick value
      FIELD_DEAL_TICK_SIZE                   =2027,         // double      , tick size
      FIELD_DEAL_FLAGS                       =2028,         // UINT64      , flags
      FIELD_DEAL_TIME_MSC                    =2029,         // INT64       , deal creation datetime in msc since 1970.01.01
      FIELD_DEAL_REASON                      =2030,         // UINT        , EnDealReason
      FIELD_DEAL_GATEWAY                     =2031,         // wchar_t[16] , source gateway name
      FIELD_DEAL_PRICE_GATEWAY               =2032,         // double      , deal price on gateway
      FIELD_DEAL_MODIFICATION_FLAGS          =2033,         // UINT        , modification flags
      FIELD_DEAL_PRICE_SL                    =2034,         // double      , order SL
      FIELD_DEAL_PRICE_TP                    =2035,         // double      , order TP
      FIELD_DEAL_FEE                         =2036,         // double      , fee
      FIELD_DEAL_VALUE                       =2037,         // double      , value
      //--- Deal fields enumeration borders
      FIELD_DEAL_FIRST                       =FIELD_DEAL_DEAL,
      FIELD_DEAL_LAST                        =FIELD_DEAL_VALUE,
      //--- Order fields enumeration
      FIELD_ORDER_ORDER                      =3001,        // UINT64      , order ticket
      FIELD_ORDER_EXTERNAL_ID                =3002,        // wchar_t[32] , order ticket in external system (exchange, ECN, etc)
      FIELD_ORDER_LOGIN                      =3003,        // UINT64      , client login
      FIELD_ORDER_DEALER                     =3004,        // UINT64      , processed dealer login (0-means auto)
      FIELD_ORDER_SYMBOL                     =3005,        // wchar_t[32] , order symbol
      FIELD_ORDER_TIME_SETUP                 =3006,        // INT64       , time of order reception from a client into the system
      FIELD_ORDER_TIME_EXPIRATION            =3007,        // INT64       , order expiration time
      FIELD_ORDER_TIME_DONE                  =3008,        // INT64       , order cancellation time
      FIELD_ORDER_TYPE                       =3009,        // UINT        , order type
      FIELD_ORDER_TYPE_FILL                  =3010,        // UINT        , order type by filling
      FIELD_ORDER_TYPE_TIME                  =3011,        // UINT        , order type by time
      FIELD_ORDER_TYPE_REASON                =3012,        // UINT        , order creation reason
      FIELD_ORDER_PRICE_ORDER                =3013,        // double      , order price
      FIELD_ORDER_PRICE_TRIGGER              =3014,        // double      , order execution price
      FIELD_ORDER_PRICE_CURRENT              =3015,        // double      , current order price
      FIELD_ORDER_PRICE_SL                   =3016,        // double      , stop-loss price
      FIELD_ORDER_PRICE_TP                   =3017,        // double      , take-profit price
      FIELD_ORDER_VOLUME_INITIAL             =3018,        // UINT64      , starting order volume
      FIELD_ORDER_VOLUME_CURRENT             =3019,        // UINT64      , current order volume
      FIELD_ORDER_STATE                      =3020,        // UINT        , actual order state
      FIELD_ORDER_EXPERT_ID                  =3021,        // UINT64      , expert ID
      FIELD_ORDER_POSITION_ID                =3022,        // UINT64      , position ID which the order opens or closes
      FIELD_ORDER_COMMENT                    =3023,        // wchar_t[32] , order comment
      FIELD_ORDER_CONTRACT_SIZE              =3024,        // double      , order contract size
      FIELD_ORDER_DIGITS                     =3025,        // UINT        , number of digits of order symbol
      FIELD_ORDER_DIGITS_CURRENCY            =3026,        // UINT        , number of digits of order currency
      FIELD_ORDER_POSITION_BY_ID             =3027,        // UINT64      , counter Position ID for Close-By orders
      FIELD_ORDER_MARGIN_RATE                =3028,        // double      , margin conversion rate at the creation time
      FIELD_ORDER_TIME_SETUP_MSC             =3029,        // INT64       , order reception from a client into the system in milliseconds
      FIELD_ORDER_TIME_DONE_MSC              =3030,        // INT64       , order cancellation time in milliseconds
      FIELD_ORDER_MODIFICATION_FLAGS         =3031,        // UINT        , order modification flags
      FIELD_ORDER_ACTIVATION_MODE            =3032,        // UINT        , order activation mode (for manager)
      FIELD_ORDER_ACTIVATION_TIME            =3033,        // INT64       , order activation time (for manager)
      FIELD_ORDER_ACTIVATION_PRICE           =3034,        // double      , order activation price (for manager)
      FIELD_ORDER_ACTIVATION_FLAGS           =3035,        // UINT        , order activation flags
      FIELD_ORDER_GROUP                      =3036,        // wchar_t[64] , group (only for open orders)
      //--- Order fields enumeration borders
      FIELD_ORDER_FIRST                      =FIELD_ORDER_ORDER,
      FIELD_ORDER_LAST                       =FIELD_ORDER_GROUP,
      //--- Daily report fields enumeration
      FIELD_DAILY_DATE_TIME                  =4001,        // INT64       , report generation date and time
      FIELD_DAILY_DATE_TIME_PREV             =4002,        // INT64       , date and time of previous report generation
      FIELD_DAILY_LOGIN                      =4003,        // UINT64      , login
      FIELD_DAILY_NAME                       =4004,        // wchar_t[128], name
      FIELD_DAILY_GROUP                      =4005,        // wchar_t[64] , group
      FIELD_DAILY_CURRENCY                   =4006,        // wchar_t[32] , currency
      FIELD_DAILY_DIGITS_CURRENCY            =4007,        // UINT        , number of digits of report currency
      FIELD_DAILY_COMPANY                    =4008,        // wchar_t[64] , company
      FIELD_DAILY_EMAIL                      =4009,        // wchar_t[64] , e-mail
      FIELD_DAILY_BALANCE                    =4010,        // double      , balance
      FIELD_DAILY_CREDIT                     =4011,        // double      , credit
      FIELD_DAILY_INTEREST_RATE              =4012,        // double      , interest rate
      FIELD_DAILY_COMMISSION_DAILY           =4013,        // double      , commission daily
      FIELD_DAILY_COMMISSION_MONTHLY         =4014,        // double      , commission monthly
      FIELD_DAILY_AGENT_DAILY                =4015,        // double      , commission daily
      FIELD_DAILY_AGENT_MONTHLY              =4016,        // double      , commission monthly
      FIELD_DAILY_BALANCE_PREV_DAY           =4017,        // double      , last day balance
      FIELD_DAILY_BALANCE_PREV_MONTH         =4018,        // double      , last month balance
      FIELD_DAILY_EQUITY_PREV_DAY            =4019,        // double      , last day equity
      FIELD_DAILY_EQUITY_PREV_MONTH          =4020,        // double      , last month equity
      FIELD_DAILY_MARGIN                     =4021,        // double      , margin
      FIELD_DAILY_MARGIN_FREE                =4022,        // double      , free margin 
      FIELD_DAILY_MARGIN_LEVEL               =4023,        // double      , margin level
      FIELD_DAILY_MARGIN_LEVERAGE            =4024,        // UINT        , margin leverage
      FIELD_DAILY_PROFIT                     =4025,        // double      , floating profit
      FIELD_DAILY_PROFIT_STORAGE             =4026,        // double      , storage
      FIELD_DAILY_PROFIT_COMMISSION          =4027,        // double      , commission
      FIELD_DAILY_PROFIT_EQUITY              =4028,        // double      , equity
      FIELD_DAILY_DAILY_PROFIT               =4029,        // double      , daily fixed profit details
      FIELD_DAILY_DAILY_BALANCE              =4030,        // double      , daily balance operations
      FIELD_DAILY_DAILY_CREDIT               =4031,        // double      , daily credit operations
      FIELD_DAILY_DAILY_CHARGE               =4032,        // double      , fees per day
      FIELD_DAILY_DAILY_CORRECTION           =4033,        // double      , correction for the day
      FIELD_DAILY_DAILY_BONUS                =4034,        // double      , bonuses per day
      FIELD_DAILY_DAILY_STORAGE              =4035,        // double      , closed positions swap
      FIELD_DAILY_DAILY_COMM_INSTANT         =4036,        // double      , commission charged immediately
      FIELD_DAILY_DAILY_COMM_ROUND           =4037,        // double      , deferred commission
      FIELD_DAILY_DAILY_AGENT                =4038,        // double      , agency commissions per day
      FIELD_DAILY_DAILY_INTEREST             =4039,        // double      , interest per day
      FIELD_DAILY_PROFIT_ASSETS              =4040,        // double      , assets
      FIELD_DAILY_PROFIT_LIABILITIES         =4041,        // double      , liabilities
      //--- Daily report fields enumeration borders
      FIELD_DAILY_FIRST                      =FIELD_DAILY_DATE_TIME,
      FIELD_DAILY_LAST                       =FIELD_DAILY_PROFIT_LIABILITIES,
      //--- Position fields enumeration
      FIELD_POSITION_LOGIN                   =5001,        // UINT64      , owner client login
      FIELD_POSITION_SYMBOL                  =5002,        // wchar_t[32] , position symbol
      FIELD_POSITION_ACTION                  =5003,        // UINT        , EnPositionAction
      FIELD_POSITION_DIGITS                  =5004,        // UINT        , number of digits of order symbol
      FIELD_POSITION_DIGITS_CURRENCY         =5005,        // UINT        , number of digits of order currency
      FIELD_POSITION_CONTRACT_SIZE           =5006,        // double      , symbol contract size
      FIELD_POSITION_TIME_CREATE             =5007,        // INT64       , position create time
      FIELD_POSITION_TIME_UPDATE             =5008,        // INT64       , position last update time
      FIELD_POSITION_PRICE_OPEN              =5009,        // double      , position weighted average open price
      FIELD_POSITION_PRICE_CURRENT           =5010,        // double      , position current price
      FIELD_POSITION_PRICE_SL                =5011,        // double      , position SL price
      FIELD_POSITION_PRICE_TP                =5012,        // double      , position TP price
      FIELD_POSITION_VOLUME                  =5013,        // UINT64      , position volume
      FIELD_POSITION_PROFIT                  =5014,        // double      , position floating profit
      FIELD_POSITION_STORAGE                 =5015,        // double      , position accumulated swaps
      FIELD_POSITION_RATE_PROFIT             =5016,        // double      , profit conversion rate (from symbol profit currency to deposit currency)
      FIELD_POSITION_RATE_MARGIN             =5017,        // double      , margin conversion rate (from symbol margin currency to deposit currency)
      FIELD_POSITION_EXPERT_ID               =5018,        // UINT64      , expert id (filled by expert advisor)
      FIELD_POSITION_EXPERT_POSITION_ID      =5019,        // UINT64      , expert position id
      FIELD_POSITION_COMMENT                 =5020,        // wchar_t[32] , comment
      FIELD_POSITION_ACTIVATION_MODE         =5021,        // UINT        , order activation state, time and price
      FIELD_POSITION_ACTIVATION_TIME         =5022,        // INT64       , activation mode
      FIELD_POSITION_ACTIVATION_PRICE        =5023,        // double      , activation time
      FIELD_POSITION_ACTIVATION_FLAGS        =5024,        // UINT        , activation flags
      FIELD_POSITION_TIME_CREATE_MSC         =5025,        // INT64       , position create time in msc since 1970.01.01
      FIELD_POSITION_TIME_UPDATE_MSC         =5026,        // INT64       , position last update time in msc since 1970.01.01
      FIELD_POSITION_DEALER                  =5027,        // UINT64      , processed dealer login (0-means auto) (first position deal dealer)
      FIELD_POSITION_POSITION                =5028,        // UINT64      , position ticket
      FIELD_POSITION_EXTERNAL_ID             =5029,        // wchar_t[32] , position ticket in external system (exchange, ECN, etc)
      FIELD_POSITION_MODIFICATION_FLAGS      =5030,        // UINT        , modification flags
      FIELD_POSITION_REASON                  =5031,        // UINT        , position reason - EnPositionReason
      FIELD_POSITION_VOLUME_EXT              =5032,        // UINT64      , position volume
      FIELD_POSITION_GROUP                   =5033,        // wchar_t[64] , group
      //--- Position fields enumeration borders
      FIELD_POSITION_FIRST                   =FIELD_POSITION_LOGIN,
      FIELD_POSITION_LAST                    =FIELD_POSITION_GROUP,
      //--- Trade account fields enumeration
      FIELD_ACCOUNT_LOGIN                    =6001,        // UINT64      , login
      FIELD_ACCOUNT_GROUP                    =6002,        // wchar_t[64] , group
      FIELD_ACCOUNT_CURRENCY_DIGITS          =6003,        // UINT        , currency digits
      FIELD_ACCOUNT_BALANCE                  =6004,        // double      , balance
      FIELD_ACCOUNT_CREDIT                   =6005,        // double      , credit
      FIELD_ACCOUNT_MARGIN                   =6006,        // double      , margin
      FIELD_ACCOUNT_MARGIN_FREE              =6007,        // double      , free margin
      FIELD_ACCOUNT_MARGIN_LEVEL             =6008,        // double      , margin level
      FIELD_ACCOUNT_MARGIN_LEVERAGE          =6009,        // UINT        , margin leverage
      FIELD_ACCOUNT_MARGIN_INITIAL           =6010,        // double      , account initial margin
      FIELD_ACCOUNT_MARGIN_MAINTENANCE       =6011,        // double      , account maintenance margin 
      FIELD_ACCOUNT_PROFIT                   =6012,        // double      , floating profit
      FIELD_ACCOUNT_STORAGE                  =6013,        // double      , storage
      FIELD_ACCOUNT_COMMISSION               =6014,        // double      , commission
      FIELD_ACCOUNT_FLOATING                 =6015,        // double      , cumulative floating
      FIELD_ACCOUNT_EQUITY                   =6016,        // double      , equity
      FIELD_ACCOUNT_BLOCKED_COMMISSION       =6017,        // double      , blocked daily & monthly commission
      FIELD_ACCOUNT_BLOCKED_PROFIT           =6018,        // double      , blocked fixed profit
      FIELD_ACCOUNT_ASSETS                   =6019,        // double      , account assets
      FIELD_ACCOUNT_LIABILITIES              =6020,        // double      , account liabilities
      FIELD_ACCOUNT_STOP_OUT_ACTIVATION      =6021,        // UINT        , stop-out activation mode
      FIELD_ACCOUNT_STOP_OUT_TIME            =6022,        // INT64       , stop-out activation time
      FIELD_ACCOUNT_STOP_OUT_LEVEL           =6023,        // double      , margin level on stop-out 
      FIELD_ACCOUNT_STOP_OUT_EQUITY          =6024,        // double      , equity on stop-out
      FIELD_ACCOUNT_STOP_OUT_MARGIN          =6025,        // double      , margin on stop-out
      //--- Trade account fields enumeration borders
      FIELD_ACCOUNT_FIRST                    =FIELD_ACCOUNT_LOGIN,
      FIELD_ACCOUNT_LAST                     =FIELD_ACCOUNT_LIABILITIES,
      //--- enumeration borders
      FIELD_FIRST                            =FIELD_USER_FIRST,
      FIELD_LAST                             =FIELD_ACCOUNT_LAST
     };
   //--- flags
   enum EnFieldFlags
     {
      FLAG_NONE                              =0x0000000,    // none
      FLAG_SELECT                            =0x0000001,    // select field
      //--- enumeration borders
      FLAG_DEFAULT                           =FLAG_SELECT,
      FLAG_ALL                               =FLAG_SELECT
     };
   //--- gender enumeration
   enum EnGender
     {
      GENDER_UNSPECIFIED                     =0,            // unspecified
      GENDER_MALE                            =1,            // male
      GENDER_FEMALE                          =2,            // female
      //--- enumeration borders
      GENDER_FIRST                           =GENDER_UNSPECIFIED,
      GENDER_LAST                            =GENDER_FEMALE
     };
   //--- client type enumeration
   enum EnClientType
     {
      CLIENT_TYPE_UNDEFINED                  =0,            // undefined
      CLIENT_TYPE_INDIVIDUAL                 =1,            // individual
      CLIENT_TYPE_CORPORATE                  =2,            // corporate
      CLIENT_TYPE_FUND                       =3,            // fund
      //--- enumeration borders
      CLIENT_TYPE_FIRST                      =CLIENT_TYPE_UNDEFINED,
      CLIENT_TYPE_LAST                       =CLIENT_TYPE_FUND
     };
   //--- client status enumeration
   enum EnClientStatus
     {
      //--- Not a client yet - Demo accounts
      CLIENT_STATUS_UNREGISTERED             =0,            // Anonymous user (demo account without data)
      CLIENT_STATUS_REGISTERED               =100,          // The user who opened a demo account and left contact information (lead)
      CLIENT_STATUS_NOTINTERESTED            =200,          // The user who left the contact information, but does not want to open an account
      //--- Not a client yet - Preliminary
      CLIENT_STATUS_APPLICATION_INCOMPLETED  =300,          // The user who filled out the form to open a real account
      CLIENT_STATUS_APPLICATION_COMPLETED    =400,          // The user who filled out the questionnaire to open a real account and provided all the documents
      CLIENT_STATUS_APPLICATION_INFORMATION  =500,          // User to open an account that requires additional information
      CLIENT_STATUS_APPLICATION_REJECTED     =600,          // The user who filled out the questionnaire to open a real account but to whom the account was not opened according to the results of the verification of documents
      //--- Client
      CLIENT_STATUS_APPROVED                 =700,          // The client who opened a real account
      CLIENT_STATUS_FUNDED                   =800,          // Customer with a real account refilled
      CLIENT_STATUS_ACTIVE                   =900,          // A client with a real account with money that made at least one transaction in the last 90 days
      CLIENT_STATUS_INACTIVE                 =1000,         // A client with a real money account that has not been traded in the past 90 days
      CLIENT_STATUS_SUSPENDED                =1100,         // Client with a real account, trading on which is suspended at the initiative of the company
      //--- No longer client
      CLIENT_STATUS_CLOSED                   =1200,         // Client who had a real account (s) and who closed them on his own initiative
      CLIENT_STATUS_TERMINATED               =1300,         // The client who had a real account (s), with which the contract was terminated at the initiative of the company
      //--- enumeration borders
      CLIENT_STATUS_FIRST                    =CLIENT_STATUS_UNREGISTERED,
      CLIENT_STATUS_LAST                     =CLIENT_STATUS_TERMINATED
     };
   //--- employment status enumeration
   enum EnEmployment
     {
      EMPLOY_UNEMPLOYED                      =0,            // unemployed
      EMPLOY_EMPLOYED                        =1,            // employed
      EMPLOY_SELF_EMPLOYED                   =2,            // self employed
      EMPLOY_RETIRED                         =3,            // retired
      EMPLOY_STUDENT                         =4,            // student
      EMPLOY_OTHER                           =5,            // other
      //--- enumeration borders
      EMPLOY_FIRST                           =EMPLOY_UNEMPLOYED,
      EMPLOY_LAST                            =EMPLOY_OTHER
     };
   //--- employment industry enumeration
   enum EnEmploymentIndustry
     {
      INDUSTRY_NONE                          =0,            // none
      INDUSTRY_AGRICULTURE                   =1,            // agriculture
      INDUSTRY_CONSTRUCTION                  =2,            // construction
      INDUSTRY_MANAGEMENT                    =3,            // management
      INDUSTRY_COMMUNICATION                 =4,            // communication
      INDUSTRY_EDUCATION                     =5,            // education
      INDUSTRY_GOVERNMENT                    =6,            // government
      INDUSTRY_HEALTHCARE                    =7,            // healthcare
      INDUSTRY_TOURISM                       =8,            // tourism
      INDUSTRY_IT                            =9,            // IT
      INDUSTRY_SECURITY                      =10,           // security
      INDUSTRY_MANUFACTURING                 =11,           // manufacturing
      INDUSTRY_MARKETING                     =12,           // marketing
      INDUSTRY_SCIENCE                       =13,           // science
      INDUSTRY_ENGINEERING                   =14,           // engineering
      INDUSTRY_TRANSPORT                     =15,           // transport
      INDUSTRY_OTHER                         =16,           // other
      //--- enumeration borders
      INDUSTRY_FIRST                         =INDUSTRY_AGRICULTURE,
      INDUSTRY_LAST                          =INDUSTRY_OTHER
     };
   //--- education level enumeration
   enum EnEducationLevel
     {
      EDUCATION_LEVEL_NONE                   =0,            // none
      EDUCATION_LEVEL_HIGH_SCHOOL            =1,            // high school
      EDUCATION_LEVEL_BACHELOR               =2,            // bachelor
      EDUCATION_LEVEL_MASTER                 =3,            // master
      EDUCATION_LEVEL_PHD                    =4,            // PhD
      EDUCATION_LEVEL_OTHER                  =5,            // other
      //--- enumeration borders
      EDUCATION_LEVEL_FIRST                  =EDUCATION_LEVEL_NONE,
      EDUCATION_LEVEL_LAST                   =EDUCATION_LEVEL_OTHER
     };
   //--- wealth source enumeration
   enum EnWealthSource
     {
      WEALTH_SOURCE_EMPLOYMENT               =0,            // employment
      WEALTH_SOURCE_SAVINGS                  =1,            // savings
      WEALTH_SOURCE_INHERITANCE              =2,            // inheritance
      WEALTH_SOURCE_OTHER                    =3,            // other
      //--- enumeration borders
      WEALTH_SOURCE_FIRST                    =WEALTH_SOURCE_EMPLOYMENT,
      WEALTH_SOURCE_LAST                     =WEALTH_SOURCE_OTHER
     };
   //--- preferred communication enumeration
   enum EnPreferredCommunication
     {
      PREFERRED_COMMUNICATION_UNDEFINED      =0,            // undefined
      PREFERRED_COMMUNICATION_EMAIL          =1,            // email
      PREFERRED_COMMUNICATION_PHONE          =2,            // phone
      PREFERRED_COMMUNICATION_PHONE_SMS      =3,            // phone SMS
      PREFERRED_COMMUNICATION_MESSENGER      =4,            // messenger
      //--- enumeration borders
      PREFERRED_COMMUNICATION_FIRST          =PREFERRED_COMMUNICATION_UNDEFINED,
      PREFERRED_COMMUNICATION_LAST           =PREFERRED_COMMUNICATION_MESSENGER
     };
   //--- trading experience enumeration
   enum EnTradingExperience
     {
      EXPERIENCE_LESS_1_YEAR                 =0,            // less than 1 year
      EXPERIENCE_1_3_YEAR                    =1,            // 1 to 3 years
      EXPERIENCE_ABOVE_3_YEAR                =2,            // above 3 years
      //--- enumeration borders
      EXPERIENCE_FIRST                       =EXPERIENCE_LESS_1_YEAR,
      EXPERIENCE_LAST                        =EXPERIENCE_ABOVE_3_YEAR
     };

public:
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTDatasetField *field)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- field id, EnFieldId
   virtual UINT      Id(void) const=0;
   virtual MTAPIRES  Id(UINT id)=0;
   //--- type, EnFieldType
   virtual UINT      Type(void) const=0;
   //--- offset in bytes
   virtual UINT      Offset(void) const=0;
   virtual MTAPIRES  Offset(UINT offset)=0;
   //--- size in bytes
   virtual UINT      Size(void) const=0;
   virtual MTAPIRES  Size(UINT size)=0;
   //--- flags, EnFieldFlags
   virtual UINT64    Flags(void) const=0;
   virtual MTAPIRES  Flags(UINT64 flags)=0;
   //--- reserved properties methods
   virtual MTAPIRES  Reserved1(void)=0;
   virtual MTAPIRES  Reserved2(void)=0;
   virtual MTAPIRES  Reserved3(void)=0;
   virtual MTAPIRES  Reserved4(void)=0;
   //--- add where value(s)
   virtual MTAPIRES  WhereAddInt(INT64 value)=0;
   virtual MTAPIRES  WhereAddIntArray(const INT64 *values,UINT values_total)=0;
   virtual MTAPIRES  WhereAddUInt(UINT64 value)=0;
   virtual MTAPIRES  WhereAddUIntArray(const UINT64 *values,UINT values_total)=0;
   virtual MTAPIRES  WhereAddDouble(double value)=0;
   virtual MTAPIRES  WhereAddDoubleArray(const double *values,UINT values_total)=0;
   virtual MTAPIRES  WhereAddString(LPCWSTR value)=0;
   virtual MTAPIRES  WhereAddStringArray(LPCWSTR *values,UINT values_total)=0;
   //--- set pointer to descending sorted array with where values
   virtual MTAPIRES  WhereUIntSet(const UINT64 *values,UINT values_total)=0;
   //--- reserved where methods
   virtual MTAPIRES  WhereReserved1(void)=0;
   virtual MTAPIRES  WhereReserved2(void)=0;
   virtual MTAPIRES  WhereReserved3(void)=0;
   //--- between values int
   virtual MTAPIRES  BetweenInt(INT64 from,INT64 to)=0;
   virtual MTAPIRES  BetweenUInt(UINT64 from,UINT64 to)=0;
   virtual MTAPIRES  BetweenDouble(double from,double to)=0;
   //--- reserved between methods
   virtual MTAPIRES  BetweenReserved1(void)=0;
   virtual MTAPIRES  BetweenReserved2(void)=0;
   virtual MTAPIRES  BetweenReserved3(void)=0;
   virtual MTAPIRES  BetweenReserved4(void)=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTDatasetField(void) {}
  };

class IMTDatasetRequest
  {
public:
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTDatasetRequest *request)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- reserved properties methods
   virtual MTAPIRES  Reserved1(void)=0;
   virtual MTAPIRES  Reserved2(void)=0;
   virtual MTAPIRES  Reserved3(void)=0;
   virtual MTAPIRES  Reserved4(void)=0;
   //--- fields
   virtual IMTDatasetField* FieldCreate(void)=0;
   virtual MTAPIRES  FieldAdd(const IMTDatasetField *field)=0;
   virtual MTAPIRES  FieldUpdate(UINT pos,const IMTDatasetField *field)=0;
   virtual MTAPIRES  FieldDelete(UINT pos)=0;
   virtual MTAPIRES  FieldClear(void)=0;
   virtual MTAPIRES  FieldShift(UINT pos,int shift)=0;
   virtual UINT      FieldTotal(void) const=0;
   virtual MTAPIRES  FieldNext(UINT pos,IMTDatasetField *field) const=0;
   virtual IMTDatasetField* FieldCreateReference(UINT pos)=0;
   //--- reserved fields methods
   virtual MTAPIRES  FieldReserved1(void)=0;
   virtual MTAPIRES  FieldReserved2(void)=0;
   virtual MTAPIRES  FieldReserved3(void)=0;
   //--- limit the number of rows of the resulting dataset
   virtual UINT      RowLimit(void) const=0;
   virtual MTAPIRES  RowLimit(UINT rows)=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTDatasetRequest(void) {}
  };

class IMTConAddressRange
  {
public:
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTConAddressRange* range)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- ip address range from
   virtual LPCWSTR   From(void) const=0;
   virtual MTAPIRES  From(LPCWSTR name)=0;
   //--- ip address range to
   virtual LPCWSTR   To(void) const=0;
   virtual MTAPIRES  To(LPCWSTR value)=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTConAddressRange(void) {}
  };

class IMTConServerAntiDDoS
  {
public:
   //--- access mask
   enum EnAccessMask
     {
      ACCESS_ALLOW_CLIENT     =1,
      ACCESS_ALLOW_MANAGER    =2,
      ACCESS_ALLOW_ADMIN      =4,
      ACCESS_ALLOW_CLIENT_API =8,
      ACCESS_ALLOW_MANAGER_API=16,
      ACCESS_ALLOW_WEB_API    =32,
      //--- enumeration borders
      ACCESS_ALLOW_NONE       =0,
      ACCESS_ALLOW_ALL        =ACCESS_ALLOW_CLIENT|ACCESS_ALLOW_MANAGER|ACCESS_ALLOW_ADMIN|ACCESS_ALLOW_CLIENT_API|ACCESS_ALLOW_MANAGER_API|ACCESS_ALLOW_WEB_API
     };
   //--- EnServerPriority
   enum EnServerPriority
     {
      PRIORITY_HIGHEST        =0,
      PRIORITY_LOWEST         =15,
      PRIORITY_IDLE           =255,
      //---
      PRIORITY_FIRST          =PRIORITY_HIGHEST,
      PRIORITY_LAST           =PRIORITY_IDLE
     };
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTConServerAntiDDoS* param)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- priority
   virtual UINT      Priority(void) const=0;
   virtual MTAPIRES  Priority(const UINT priority)=0;
   //--- access flags EnAccessMask
   virtual UINT      AccessMask(void) const=0;
   virtual MTAPIRES  AccessMask(const UINT mask)=0;
   //--- list of public addresses (address:port) available for terminals
   virtual MTAPIRES  PointsAdd(LPCWSTR path)=0;
   virtual MTAPIRES  PointsUpdate(const UINT pos,LPCWSTR address)=0;
   virtual MTAPIRES  PointsDelete(const UINT pos)=0;
   virtual MTAPIRES  PointsClear(void)=0;
   virtual MTAPIRES  PointsShift(const UINT pos,const int shift)=0;
   virtual UINT      PointsTotal(void) const=0;
   virtual LPCWSTR   PointsNext(const UINT pos) const=0;
   //--- list of trade attended trade servers
   virtual MTAPIRES  ServersAdd(const UINT64 server_id)=0;
   virtual MTAPIRES  ServersUpdate(const UINT pos,UINT64 server_id)=0;
   virtual MTAPIRES  ServersDelete(const UINT pos)=0;
   virtual MTAPIRES  ServersClear(void)=0;
   virtual MTAPIRES  ServersShift(const UINT pos,const int shift)=0;
   virtual UINT      ServersTotal(void) const=0;
   virtual UINT64    ServersNext(const UINT pos) const=0;
   //--- list of source addresses
   virtual MTAPIRES  SourcesAdd(IMTConAddressRange* range)=0;
   virtual MTAPIRES  SourcesUpdate(const UINT pos,const IMTConAddressRange* range)=0;
   virtual MTAPIRES  SourcesDelete(const UINT pos)=0;
   virtual MTAPIRES  SourcesShift(const UINT pos,const int shift)=0;
   virtual UINT      SourcesTotal(void) const=0;
   virtual MTAPIRES  SourcesNext(const UINT pos,IMTConAddressRange* access) const=0;
protected:
                    ~IMTConServerAntiDDoS(void) {}
  };

class IMTConServerBackup
  {
public:
   //--- backup flags
   enum EnBackupFlags
     {
      FLAG_ENABLE_BACKUPS =0x00000001,
      FLAG_ENABLE_TICKS   =0x00000002,
      FLAG_ENABLE_FAILOVER=0x00000004,
      FLAG_ENABLE_LOGS    =0x00000008
     };
   //--- backup period
   enum EnBackupPeriod
     {
      BACKUP_DISABLED   =0,
      BACKUP_15MINUTES  =1,
      BACKUP_30MINUTES  =2,
      BACKUP_1HOUR      =3,
      BACKUP_4HOURS     =4,
      BACKUP_1DAY       =5,
      //--- enumeration borders
      BACKUP_FIRST      =BACKUP_DISABLED,
      BACKUP_LAST       =BACKUP_1DAY
     };
   //--- backup copy time to live
   enum EnBackupTTL
     {
      BACKUP_TTL_1DAY   =1,
      BACKUP_TTL_3DAYS  =2,
      BACKUP_TTL_1WEEK  =3,
      BACKUP_TTL_1MONTH =4,
      BACKUP_TTL_3MONTHS=5,
      BACKUP_TTL_6MONTHS=6,
      //--- enumeration borders
      BACKUP_TTL_FIRST  =BACKUP_TTL_1DAY,
      BACKUP_TTL_LAST   =BACKUP_TTL_6MONTHS
     };
   //--- SQL export modes
   enum EnSQLExportMode
     {
      SQL_MODE_NONE      =0,
      SQL_MODE_MSSQL     =1,
      SQL_MODE_FIREBIRD  =2,
      SQL_MODE_MYSQL     =3,
      SQL_MODE_ORACLE    =4,
      SQL_MODE_POSTGRESQL=5,
      //--- enumeration borders
      SQL_MODE_FIRST    =SQL_MODE_NONE,
      SQL_MODE_LAST     =SQL_MODE_POSTGRESQL,
     };
   //--- SQL export flags
   enum EnSQLExportFlags
     {
      SQL_FLAG_NONE      =0x00000000,
      SQL_FLAG_PARTITIONS=0x00000001, // split by years
      SQL_FLAG_SKIP_DEMO =0x00000002, // skip demo accounts
      //--- enumeration borders
      SQL_FLAG_ALL       =SQL_FLAG_PARTITIONS|SQL_FLAG_SKIP_DEMO
     };
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTConServerBackup* param)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- master server
   virtual UINT64    MasterServer(void) const=0;
   virtual MTAPIRES  MasterServer(const UINT64 id)=0;
   //--- backup path
   virtual LPCWSTR   BackupPath(void) const=0;
   virtual MTAPIRES  BackupPath(LPCWSTR path)=0;
   //--- full backup time (minutes)
   virtual UINT      BackupFullTime(void) const=0;
   virtual MTAPIRES  BackupFullTime(const UINT time)=0;
   //--- backup period - EnBackupPeriod
   virtual UINT      BackupPeriod(void) const=0;
   virtual MTAPIRES  BackupPeriod(const UINT period)=0;
   //--- backup copy TTL - EnBackupTTL
   virtual UINT      BackupTTL(void) const=0;
   virtual MTAPIRES  BackupTTL(const UINT period)=0;
   //--- backup flags - EnBackupFlags
   virtual UINT64    BackupFlags(void) const=0;
   virtual MTAPIRES  BackupFlags(const UINT64 flags)=0;
   //--- sql export mode
   virtual UINT      SQLExportMode(void) const=0;
   virtual MTAPIRES  SQLExportMode(const UINT mode)=0;
   //--- sql export flags
   virtual UINT64    SQLExportFlags(void) const=0;
   virtual MTAPIRES  SQLExportFlags(const UINT64 flags)=0;
   //--- sql export open trades refresh period in minutes
   virtual UINT      SQLExportPeriod(void) const=0;
   virtual MTAPIRES  SQLExportPeriod(const UINT period)=0;
   //--- sql export server
   virtual LPCWSTR   SQLExportServer(void) const=0;
   virtual MTAPIRES  SQLExportServer(LPCWSTR server)=0;
   //--- sql export login
   virtual LPCWSTR   SQLExportLogin(void) const=0;
   virtual MTAPIRES  SQLExportLogin(LPCWSTR login)=0;
   //--- sql export password
   virtual LPCWSTR   SQLExportPassword(void) const=0;
   virtual MTAPIRES  SQLExportPassword(LPCWSTR password)=0;
   //--- sql export folder
   virtual LPCWSTR   SQLExportFolder(void) const=0;
   virtual MTAPIRES  SQLExportFolder(LPCWSTR folder)=0;
   //--- last synchronization with master server
   virtual INT64     BackupLastSync(void) const=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTConServerBackup(void) {}
  };

class IMTConServerAccess
  {
public:
   //--- access mask
   enum EnAccessMask
     {
      ACCESS_ALLOW_CLIENT     =1,
      ACCESS_ALLOW_MANAGER    =2,
      ACCESS_ALLOW_ADMIN      =4,
      ACCESS_ALLOW_RESERVED   =8,
      ACCESS_ALLOW_MANAGER_API=16,
      ACCESS_ALLOW_WEB_API    =32,
      //--- enumeration borders
      ACCESS_ALLOW_NONE       =0,
      ACCESS_ALLOW_ALL        =ACCESS_ALLOW_CLIENT|ACCESS_ALLOW_MANAGER|ACCESS_ALLOW_ADMIN|ACCESS_ALLOW_RESERVED|ACCESS_ALLOW_MANAGER_API|ACCESS_ALLOW_WEB_API
     };
   //--- access flags
   enum EnAccessFlags
     {
      ACCESS_FLAGS_INVISIBLE  =1,
      //--- enumeration borders
      ACCESS_FLAGS_NONE       =0,
      ACCESS_FLAGS_ALL        =ACCESS_FLAGS_INVISIBLE
     };
   //--- EnServerPriority
   enum EnServerPriority
     {
      PRIORITY_HIGHEST        =0,
      PRIORITY_LOWEST         =15,
      PRIORITY_IDLE           =255,
      //---
      PRIORITY_FIRST          =PRIORITY_HIGHEST,
      PRIORITY_LAST           =PRIORITY_IDLE
     };
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTConServerAccess* param)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- priority
   virtual UINT      Priority(void) const=0;
   virtual MTAPIRES  Priority(const UINT priority)=0;
   virtual UINT      PriorityCurrent(void) const=0;
   //--- access flags EnAccessMask
   virtual UINT      AccessMask(void) const=0;
   virtual MTAPIRES  AccessMask(const UINT mask)=0;
   //--- max news buffer
   virtual UINT      NewsMax(void) const=0;
   virtual MTAPIRES  NewsMax(const UINT news_max)=0;
   //--- antiflood
   virtual UINT      AntifloodEnabled(void) const=0;
   virtual MTAPIRES  AntifloodEnabled(const UINT enabled)=0;
   //--- antiflood connects criterion
   virtual UINT      AntifloodConnects(void) const=0;
   virtual MTAPIRES  AntifloodConnects(const UINT connects)=0;
   //--- antiflood connects criterion
   virtual UINT      AntifloodErrors(void) const=0;
   virtual MTAPIRES  AntifloodErrors(const UINT errors)=0;
   //--- list of public addresses (address:port) available for terminals
   virtual MTAPIRES  PointsAdd(LPCWSTR path)=0;
   virtual MTAPIRES  PointsUpdate(const UINT pos,LPCWSTR address)=0;
   virtual MTAPIRES  PointsDelete(const UINT pos)=0;
   virtual MTAPIRES  PointsClear(void)=0;
   virtual MTAPIRES  PointsShift(const UINT pos,const int shift)=0;
   virtual UINT      PointsTotal(void) const=0;
   virtual LPCWSTR   PointsNext(const UINT pos) const=0;
   //--- list of internal addresses (address:port) for server listening
   virtual MTAPIRES  BindingsAdd(LPCWSTR path)=0;
   virtual MTAPIRES  BindingsUpdate(const UINT pos,LPCWSTR address)=0;
   virtual MTAPIRES  BindingsDelete(const UINT pos)=0;
   virtual MTAPIRES  BindingsClear(void)=0;
   virtual MTAPIRES  BindingsShift(const UINT pos,const int shift)=0;
   virtual UINT      BindingsTotal(void) const=0;
   virtual LPCWSTR   BindingsNext(const UINT pos) const=0;
   //--- list of trade attended trade servers
   virtual MTAPIRES  ServersAdd(const UINT64 server_id)=0;
   virtual MTAPIRES  ServersUpdate(const UINT pos,UINT64 server_id)=0;
   virtual MTAPIRES  ServersDelete(const UINT pos)=0;
   virtual MTAPIRES  ServersClear(void)=0;
   virtual MTAPIRES  ServersShift(const UINT pos,const int shift)=0;
   virtual UINT      ServersTotal(void) const=0;
   virtual UINT64    ServersNext(const UINT pos) const=0;
   //--- access flags EnAccessFlags
   virtual UINT      AccessFlags(void) const=0;
   virtual MTAPIRES  AccessFlags(const UINT flags)=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTConServerAccess(void) {}
  };

class IMTConServerHistory
  {
public:
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTConServerHistory* param)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- datafeeds switch timeout
   virtual UINT      DatafeedsTimeout(void) const=0;
   virtual MTAPIRES  DatafeedsTimeout(const UINT timeout)=0;
   //--- max news buffer
   virtual UINT      NewsMax(void) const=0;
   virtual MTAPIRES  NewsMax(const UINT news_max)=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTConServerHistory(void) {}
  };

class IMTConServerRange
  {
public:
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTConServerRange* param)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- from
   virtual UINT64    From(void) const=0;
   virtual MTAPIRES  From(const UINT64 from)=0;
   //--- to
   virtual UINT64    To(void) const=0;
   virtual MTAPIRES  To(const UINT64 to)=0;
   //--- used range
   virtual UINT64    UsedFrom(void) const=0;
   virtual UINT64    UsedTo(void) const=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTConServerRange(void) {}
  };

class IMTConServerTrade
  {
public:
   //--- demo account allocation mode
   enum EnDemoMode
     {
      DEMO_DISABLED         =0, // demo account allocation disabled 
      DEMO_PROLONG          =1, // prolong demo acounts after reconnect
      DEMO_FIXED            =2, // demo account with fixed period
      //--- enumeration borders
      DEMO_FIRST            =DEMO_DISABLED,
      DEMO_LAST             =DEMO_FIXED
     };
   //--- enumeration borders
   enum EnOvernightMode
     {
      OVERNIGHT_END_DAY     =0, // overnight performed at the end of the trading day
      OVERNIGHT_START_DAY   =1, // overnight performed at the begin of the trading day
      //--- enumeration borders
      OVERNIGHT_FIRST       =OVERNIGHT_END_DAY,
      OVERNIGHT_LAST        =OVERNIGHT_START_DAY
     };
   //--- overmonth mode 
   enum EnOvermonthMode
     {
      OVERMONTH_LAST_DAY    =0, // overmonth performed at the last month day
      OVERMONTH_FIRST_DAY   =1, // overmonth performed at the first month day
      //--- enumeration borders
      OVERMONTH_FIRST       =OVERMONTH_LAST_DAY,
      OVERMONTH_LAST        =OVERMONTH_FIRST_DAY
     };
   //--- overnigh days enumeration
   enum EnOvernightDays
     {
      OVERNIGHT_DAYS_SUN         =0x00000001,
      OVERNIGHT_DAYS_MON         =0x00000002,
      OVERNIGHT_DAYS_TUE         =0x00000004,
      OVERNIGHT_DAYS_WED         =0x00000008,
      OVERNIGHT_DAYS_THU         =0x00000010,
      OVERNIGHT_DAYS_FRI         =0x00000020,
      OVERNIGHT_DAYS_SAT         =0x00000040,
      //--- rollovers schedule
      OVERNIGHT_DAYS_ROLLOVER_SUN=0x00000080,
      OVERNIGHT_DAYS_ROLLOVER_MON=0x00000100,
      OVERNIGHT_DAYS_ROLLOVER_TUE=0x00000200,
      OVERNIGHT_DAYS_ROLLOVER_WED=0x00000400,
      OVERNIGHT_DAYS_ROLLOVER_THU=0x00000800,
      OVERNIGHT_DAYS_ROLLOVER_FRI=0x00001000,
      OVERNIGHT_DAYS_ROLLOVER_SAT=0x00002000,
      //--- enumeration borders
      OVERNIGHT_DAYS_NONE        =0x00000000,
      OVERNIGHT_DAYS_DEFAULT =OVERNIGHT_DAYS_MON | OVERNIGHT_DAYS_TUE | OVERNIGHT_DAYS_WED | OVERNIGHT_DAYS_THU | OVERNIGHT_DAYS_FRI |
      OVERNIGHT_DAYS_ROLLOVER_MON | OVERNIGHT_DAYS_ROLLOVER_TUE | OVERNIGHT_DAYS_ROLLOVER_WED | OVERNIGHT_DAYS_ROLLOVER_THU | OVERNIGHT_DAYS_ROLLOVER_FRI,
      OVERNIGHT_DAYS_ALL     =OVERNIGHT_DAYS_SUN | OVERNIGHT_DAYS_MON | OVERNIGHT_DAYS_TUE | OVERNIGHT_DAYS_WED | OVERNIGHT_DAYS_THU | OVERNIGHT_DAYS_FRI | OVERNIGHT_DAYS_SAT |
      OVERNIGHT_DAYS_ROLLOVER_SUN | OVERNIGHT_DAYS_ROLLOVER_MON | OVERNIGHT_DAYS_ROLLOVER_TUE | OVERNIGHT_DAYS_ROLLOVER_WED | OVERNIGHT_DAYS_ROLLOVER_THU |
      OVERNIGHT_DAYS_ROLLOVER_FRI | OVERNIGHT_DAYS_ROLLOVER_SAT
     };
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTConServerTrade* param)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- demo accounts mode EnDemoMode
   virtual UINT      DemoMode(void) const=0;
   virtual MTAPIRES  DemoMode(const UINT mode)=0;
   //--- demo accounts period
   virtual UINT      DemoPeriod(void) const=0;
   virtual MTAPIRES  DemoPeriod(const UINT period)=0;
   //--- overnight mode
   virtual UINT      OvernightMode(void) const=0;
   virtual MTAPIRES  OvernightMode(const UINT mode)=0;
   //--- overnight time (in minutes)
   virtual UINT      OvernightTime(void) const=0;
   virtual MTAPIRES  OvernightTime(const UINT time)=0;
   virtual INT64     OvernightTimeLast(void) const=0;
   virtual INT64     OvernightTimePrev(void) const=0;
   //--- overnight mode
   virtual UINT      OvermonthMode(void) const=0;
   virtual MTAPIRES  OvermonthMode(const UINT mode)=0;
   //--- overmonth time (in minutes)
   virtual INT64     OvermonthTimeLast(void) const=0;
   virtual INT64     OvermonthTimePrev(void) const=0;
   //--- client logins ranges for this trade server
   virtual MTAPIRES  LoginsRangeAdd(IMTConServerRange* range)=0;
   virtual MTAPIRES  LoginsRangeUpdate(const UINT pos,const IMTConServerRange* range)=0;
   virtual MTAPIRES  LoginsRangeDelete(const UINT pos)=0;
   virtual MTAPIRES  LoginsRangeClear(void)=0;
   virtual MTAPIRES  LoginsRangeShift(const UINT pos,const int shift)=0;
   virtual UINT      LoginsRangeTotal(void) const=0;
   virtual MTAPIRES  LoginsRangeNext(const UINT pos,IMTConServerRange* range) const=0;
   //--- orders tickets ranges for this trade server
   virtual MTAPIRES  OrdersRangeAdd(IMTConServerRange* range)=0;
   virtual MTAPIRES  OrdersRangeUpdate(const UINT pos,const IMTConServerRange* range)=0;
   virtual MTAPIRES  OrdersRangeDelete(const UINT pos)=0;
   virtual MTAPIRES  OrdersRangeClear(void)=0;
   virtual MTAPIRES  OrdersRangeShift(const UINT pos,const int shift)=0;
   virtual UINT      OrdersRangeTotal(void) const=0;
   virtual MTAPIRES  OrdersRangeNext(const UINT pos,IMTConServerRange* range) const=0;
   //--- deals tickets ranges for this trade server
   virtual MTAPIRES  DealsRangeAdd(IMTConServerRange* range)=0;
   virtual MTAPIRES  DealsRangeUpdate(const UINT pos,const IMTConServerRange* range)=0;
   virtual MTAPIRES  DealsRangeDelete(const UINT pos)=0;
   virtual MTAPIRES  DealsRangeClear(void)=0;
   virtual MTAPIRES  DealsRangeShift(const UINT pos,const int shift)=0;
   virtual UINT      DealsRangeTotal(void) const=0;
   virtual MTAPIRES  DealsRangeNext(const UINT pos,IMTConServerRange* range) const=0;
   //--- trade totals
   virtual UINT      TotalUsers(void) const=0;
   virtual UINT      TotalUsersReal(void) const=0;
   virtual UINT      TotalDeals(void) const=0;
   virtual UINT      TotalOrders(void) const=0;
   virtual UINT      TotalOrdersHistory(void) const=0;
   virtual UINT      TotalPositions(void) const=0;
   //--- overnight days EnOvernightDays
   virtual UINT      OvernightDays(void) const=0;
   virtual MTAPIRES  OvernightDays(const UINT days)=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTConServerTrade(void) {}
  };

class IMTConClusterState
  {
public:
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTConClusterState* state)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- server id
   virtual UINT64    Id(void) const=0;
   //--- connection state
   virtual bool      Connected(void) const=0;
   virtual LPCWSTR   ConnectedAddress(void) const=0;
   virtual INT64     ConnectedTime(void) const=0;
   //--- connection stats
   virtual INT64     StatsDay(void) const=0;
   virtual UINT      StatsPing(void) const=0;
   virtual UINT      StatsPingMin(void) const=0;
   virtual UINT      StatsPingMax(void) const=0;
   virtual UINT      StatsSpeed(void) const=0;
   virtual UINT      StatsSpeedMin(void) const=0;
   virtual UINT      StatsSpeedMax(void) const=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTConClusterState(void) {}
  };

class IMTConServer
  {
public:
   //--- server types
   enum EnServerTypes
     {
      NET_MAIN_TRADE_SERVER   =0,
      NET_TRADE_SERVER        =1,
      NET_HISTORY_SERVER      =2,
      NET_ACCESS_SERVER       =3,
      NET_BACKUP_SERVER       =4,
      NET_OBSOLETE_SERVER_1   =5,
      NET_OBSOLETE_SERVER_2   =6,
      NET_ANTIDDOS_SERVER     =7,
      //--- enumeration borders
      NET_SERVER_FIRST        =NET_MAIN_TRADE_SERVER,
      NET_SERVER_LAST         =NET_ANTIDDOS_SERVER
     };
   //--- failover mode
   enum EnFailoverModes
     {
      FAILOVER_MODE_DISABLED  =0,  // disabled
      FAILOVER_MODE_BY_MOST   =1,  // server is unaccessible for most witness servers
      FAILOVER_MODE_BY_ALL    =2,  // server is unaccessible for all witness servers
      //--- enumeration borders
      FAILOVER_MODE_FIRST     =FAILOVER_MODE_DISABLED,
      FAILOVER_MODE_LAST      =FAILOVER_MODE_BY_ALL
     };
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTConServer* param)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- server type
   virtual UINT      Type(void) const=0;
   virtual MTAPIRES  Type(const UINT type)=0;
   //--- server name
   virtual LPCWSTR   Name(void) const=0;
   virtual MTAPIRES  Name(LPCWSTR name)=0;
   //--- address 
   virtual LPCWSTR   Address(void) const=0;
   virtual MTAPIRES  Address(LPCWSTR name)=0;
   //--- server id
   virtual UINT64    Id(void) const=0;
   virtual MTAPIRES  Id(const UINT64 id)=0;
   //--- password
   virtual MTAPIRES  Password(LPCWSTR password)=0;
   virtual MTAPIRES  PasswordCheck(LPCWSTR password) const=0;
   //--- service time in minutes
   virtual UINT      ServiceTime(void) const=0;
   virtual MTAPIRES  ServiceTime(const UINT stime)=0;
   //--- list of network adapters
   virtual LPCWSTR   AdaptersCurrent(void) const=0;
   virtual MTAPIRES  AdaptersCurrent(LPCWSTR current)=0;
   virtual UINT      AdaptersTotal(void) const=0;
   virtual LPCWSTR   AdaptersNext(const UINT pos) const=0;
   //--- list of available ips
   virtual UINT      AddressTotal(void) const=0;
   virtual UINT      AddressNext(const UINT pos) const=0;
   //--- server info
   virtual UINT      Version(void) const=0;
   virtual UINT      Build(void) const=0;
   virtual LPCWSTR   BuildDate(void) const=0;
   virtual INT64     LastBootTime(void) const=0;
   virtual bool      Connected(void) const=0;
   virtual LPCWSTR   OS(void) const=0;
   //--- cpu info
   virtual LPCWSTR   CPU(void) const=0;
   virtual UINT      CPUTotal(void) const=0;
   virtual UINT      CPUUsageMax(void) const=0;
   virtual UINT      CPUUsageCritical(void) const=0;
   //--- memory info (in MB)
   virtual UINT      MemoryTotal(void) const=0;
   virtual UINT      MemoryFree(void) const=0;
   virtual UINT      MemoryFreeMin(void) const=0;
   virtual UINT      MemoryFreeCritical(void) const=0;
   //--- hdd info (in MB)
   virtual UINT      HDDTotal(void) const=0;
   virtual UINT      HDDFree(void) const=0;
   virtual UINT      HDDFreeCritical(void) const=0;
   virtual UINT      HDDFragments(void) const=0;
   virtual UINT      HDDFragmentsCritical(void) const=0;
   virtual UINT      HDDSpeedRead(void) const=0;
   virtual UINT      HDDSpeedReadCritical(void) const=0;
   virtual UINT      HDDSpeedWrite(void) const=0;
   virtual UINT      HDDSpeedWriteCritical(void) const=0;
   //--- connections info
   virtual UINT      ConnectsMax(void) const=0;
   virtual UINT      ConnectsCritical(void) const=0;
   //--- network info (Kbyte/s)
   virtual UINT      NetworkMax(void) const=0;
   virtual UINT      NetworkCritical(void) const=0;
   //--- specific server interfaces
   virtual IMTConServerTrade* TradeServer(void)=0;
   virtual IMTConServerHistory* HistoryServer(void)=0;
   virtual IMTConServerAccess* AccessServer(void)=0;
   virtual IMTConServerBackup* BackupServer(void)=0;
   virtual IMTConServerAntiDDoS* AntiDDoSServer(void)=0;
   virtual void*     ReservedServer1(void)=0;
   virtual void*     ReservedServer2(void)=0;
   virtual void*     ReservedServer3(void)=0;
   virtual void*     ReservedServer4(void)=0;
   //--- list of external addresses (address:port) for connections to server
   virtual MTAPIRES  PointsAdd(LPCWSTR path)=0;
   virtual MTAPIRES  PointsUpdate(const UINT pos,LPCWSTR address)=0;
   virtual MTAPIRES  PointsDelete(const UINT pos)=0;
   virtual MTAPIRES  PointsClear(void)=0;
   virtual MTAPIRES  PointsShift(const UINT pos,const int shift)=0;
   virtual UINT      PointsTotal(void) const=0;
   virtual LPCWSTR   PointsNext(const UINT pos) const=0;
   //--- list of internal addresses (address:port) for server listening
   virtual MTAPIRES  BindingsAdd(LPCWSTR path)=0;
   virtual MTAPIRES  BindingsUpdate(const UINT pos,LPCWSTR address)=0;
   virtual MTAPIRES  BindingsDelete(const UINT pos)=0;
   virtual MTAPIRES  BindingsClear(void)=0;
   virtual MTAPIRES  BindingsShift(const UINT pos,const int shift)=0;
   virtual UINT      BindingsTotal(void) const=0;
   virtual LPCWSTR   BindingsNext(const UINT pos) const=0;
   //--- failover mode
   virtual UINT      FailoverMode(void) const=0;
   virtual MTAPIRES  FailoverMode(const UINT mode)=0;
   //--- failover timeout
   virtual UINT      FailoverTimeout(void) const=0;
   virtual MTAPIRES  FailoverTimeout(const UINT timeout)=0;
   //--- cluster servers state
   virtual UINT      ClusterStateTotal(void) const=0;
   virtual MTAPIRES  ClusterStateNext(const UINT pos,IMTConClusterState* state) const=0;
   virtual MTAPIRES  ClusterStateGet(const UINT64 id,IMTConClusterState* state) const=0;
   //--- address
   virtual LPCWSTR   AddressIPv6(void) const=0;
   virtual MTAPIRES  AddressIPv6(LPCWSTR name)=0;
   //--- list of available ips
   virtual UINT      AddressIPv6Total(void) const=0;
   virtual MTAPIRES  AddressIPv6Next(const UINT pos,MTAPISTR& address) const=0;

protected:
                    ~IMTConServer(void) {}
  };

class IMTConTime
  {
public:
   //--- day working mode
   enum EnTimeTableMode
     {
      TIME_MODE_DISABLED=0,   // work enabled
      TIME_MODE_ENABLED =1,   // work disabled
      //---
      TIME_MODE_FIRST   =TIME_MODE_DISABLED,
      TIME_MODE_LAST    =TIME_MODE_ENABLED
     };
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(IMTConTime* param)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- daylight correction mode
   virtual bool      Daylight(void) const=0;
   virtual MTAPIRES  Daylight(const bool enable)=0;
   //--- server timezone in minutes (0=GMT;-60=GMT-1;60=GMT+1)
   virtual int       TimeZone(void) const=0;
   virtual MTAPIRES  TimeZone(const int zone)=0;
   //--- time synchronization server address (TIME or NTP protocol)
   virtual LPCWSTR   TimeServer(void) const=0;
   virtual MTAPIRES  TimeServer(LPCWSTR server)=0;
   //--- time schedule (wday -> 0-Sunday,6-Saturday, hour-> 0-24, mode-> EnTimeTableMode)
   virtual MTAPIRES  TimeTableGet(const UINT wday,const UINT hour,UINT& mode) const=0;
   virtual MTAPIRES  TimeTableSet(const UINT wday,const UINT hour,const UINT mode)=0;
   //--- current daylight state
   virtual int       DaylightState(void) const=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTConTime(void) {}
  };

class IMTConTimeSink
  {
public:
   virtual void      OnTimeUpdate(const IMTConTime* /*config*/) {  }
   virtual void      OnTimeSync(void)                           {  }
  };

class IMTConServerSink
  {
public:
   virtual void      OnConServerAdd(const IMTConServer*    /*server*/) {  }
   virtual void      OnConServerUpdate(const IMTConServer* /*server*/) {  }
   virtual void      OnConServerDelete(const IMTConServer* /*server*/) {  }
   virtual void      OnConServerSync(void)                             {  }
  };

class IMTConCommon
  {
public:
   //--- LiveUpdate modes
   enum EnUpdateMode
     {
      UPDATE_DISABLE    =0,  // disable LiveUpdate
      UPDATE_ENABLE     =1,  // enable LiveUpdate
      UPDATE_ENABLE_BETA=2,  // enable LiveUpdate, including beta releases
      //--- enumeration borders
      UPDATE_FIRST      =UPDATE_DISABLE,
      UPDATE_LAST       =UPDATE_ENABLE_BETA
     };
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTConCommon* param)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- server name
   virtual LPCWSTR   Name(void) const=0;
   virtual MTAPIRES  Name(LPCWSTR name)=0;
   //--- server full name
   virtual LPCWSTR   NameFull(void) const=0;
   //--- owner full name (from license)
   virtual LPCWSTR   Owner(void) const=0;
   //--- owner short name (from license)
   virtual LPCWSTR   OwnerID(void) const=0;
   //--- owner host (from license)
   virtual LPCWSTR   OwnerHost(void) const=0;
   //--- owner email (from license)
   virtual LPCWSTR   OwnerEmail(void) const=0;
   //--- product full name (from license)
   virtual LPCWSTR   Product(void) const=0;
   //--- license expiration date
   virtual INT64     ExpirationLicense(void) const=0;
   //--- license support date
   virtual INT64     ExpirationSupport(void) const=0;
   //--- max. trade servers count (from license)
   virtual UINT      LimitTradeServers(void) const=0;
   //--- max. web servers count (from license)
   virtual UINT      LimitWebServers(void) const=0;
   //--- max. real accounts count (from license)
   virtual UINT      LimitAccounts(void) const=0;
   //--- max. trade deals count (from license)
   virtual UINT      LimitDeals(void) const=0;
   //--- max. client groups count (from license)
   virtual UINT      LimitGroups(void) const=0;
   //--- LiveUpdate mode
   virtual UINT      LiveUpdateMode(void) const=0;
   virtual MTAPIRES  LiveUpdateMode(const UINT mode)=0;
   //--- Total users
   virtual UINT      TotalUsers(void) const=0;
   //--- Total real users
   virtual UINT      TotalUsersReal(void) const=0;
   //--- Total deals
   virtual UINT      TotalDeals(void) const=0;
   //--- Total orders
   virtual UINT      TotalOrders(void) const=0;
   //--- Total history orders
   virtual UINT      TotalOrdersHistory(void) const=0;
   //--- Total positions
   virtual UINT      TotalPositions(void) const=0;
   //--- max. symbols count (from license)
   virtual UINT      LimitSymbols(void) const=0;
   //--- Account Allocation URL
   virtual LPCWSTR   AccountURL(void) const=0;
   virtual MTAPIRES  AccountURL(LPCWSTR url)=0;
   //--- Account Deposit URL
   virtual LPCWSTR   AccountDepositURL(void) const=0;
   virtual MTAPIRES  AccountDepositURL(LPCWSTR url)=0;
   //--- Account Withdrawal URL
   virtual LPCWSTR   AccountWithdrawalURL(void) const=0;
   virtual MTAPIRES  AccountWithdrawalURL(LPCWSTR url)=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTConCommon(void) {}
  };

class IMTConCommonSink
  {
public:
   virtual void      OnCommonUpdate(const IMTConCommon* /*config*/) {  }
   virtual void      OnCommonSync(void)                             {  }
  };

class IMTConParam
  {
public:
   //--- parameter types
   enum ParamType
     {
      TYPE_STRING    =0,   // string
      TYPE_INT       =1,   // integer
      TYPE_FLOAT     =2,   // floating
      TYPE_TIME      =3 ,  // time only
      TYPE_DATE      =4 ,  // date only
      TYPE_DATETIME  =5 ,  // date & time
      TYPE_GROUPS    =6,   // groups list
      TYPE_SYMBOLS   =7,   // symbols list
      //--- enumeration borders
      TYPE_FIRST     =TYPE_STRING,
      TYPE_LAST      =TYPE_SYMBOLS
     };
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTConParam* param)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- parameter name
   virtual LPCWSTR   Name(void) const=0;
   virtual MTAPIRES  Name(LPCWSTR name)=0;
   //--- parameter type
   virtual UINT      Type(void) const=0;
   virtual MTAPIRES  Type(const UINT type)=0;
   //--- parameter value (raw string)
   virtual LPCWSTR   Value(void) const=0;
   virtual MTAPIRES  Value(LPCWSTR value)=0;
   //--- parameter value string
   virtual LPCWSTR   ValueString(void) const=0;
   virtual MTAPIRES  ValueString(LPCWSTR value)=0;
   //--- parameter value integer
   virtual INT64     ValueInt(void) const=0;
   virtual MTAPIRES  ValueInt(const INT64 value)=0;
   //--- parameter value float
   virtual double    ValueFloat(void) const=0;
   virtual MTAPIRES  ValueFloat(const double value)=0;
   //--- parameter value time
   virtual INT64     ValueTime(void) const=0;
   virtual MTAPIRES  ValueTime(const INT64 value)=0;
   //--- parameter value datetime
   virtual INT64     ValueDatetime(void) const=0;
   virtual MTAPIRES  ValueDatetime(const INT64 value)=0;
   //--- parameter value groups
   virtual LPCWSTR   ValueGroups(void) const=0;
   virtual MTAPIRES  ValueGroups(LPCWSTR value)=0;
   //--- parameter value symbols
   virtual LPCWSTR   ValueSymbols(void) const=0;
   virtual MTAPIRES  ValueSymbols(LPCWSTR value)=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTConParam(void) {}
  };

class IMTConPluginModule
  {
public:
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTConPluginModule* param)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- default plugin name
   virtual LPCWSTR   Name(void) const=0;
   //--- vendor name
   virtual LPCWSTR   Vendor(void) const=0;
   //--- plugin description
   virtual LPCWSTR   Description(void) const=0;
   //--- plugin file name
   virtual LPCWSTR   Module(void) const=0;
   //--- MT5 server-owner id
   virtual UINT64    Server(void) const=0;
   //--- plugin version
   virtual UINT      Version(void) const=0;
   //--- plugin Server API version
   virtual UINT      VersionAPI(void) const=0;
   //--- plugin default parameters
   virtual UINT      ParameterTotal(void) const=0;
   virtual MTAPIRES  ParameterNext(const UINT pos,IMTConParam* param) const=0;
   virtual MTAPIRES  ParameterGet(LPCWSTR name,IMTConParam* param) const=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTConPluginModule(void) {}
  };

#pragma pack(push,1)
struct MTLogRecord
  {
   UINT              flags;                     // flags EnMTLogFlags
   INT               code;                      // code EnMTLogCode
   UINT              type;                      // type EnMTLogType
   INT64             datetime;                  // record time
   wchar_t           source[64];                // record source
   wchar_t           message[512];              // record message
   INT64             datetime_msc;              // record time in milliseconds since 1970.01.01
   int               reserved[2];               // reserved
  };
#pragma pack(pop)

class IMTServerSink
  {
public:
   virtual void      OnServerLog(const INT /*code*/,const UINT /*type*/,const INT64 /*datetime_msc*/,LPCWSTR /*source*/,LPCWSTR /*message*/){}
  };

class IMTConPlugin
  {
public:
   //--- plugin working flags
   enum EnPluginFlags
     {
      PLUGIN_FLAG_MAN_CONFIG=1,  // allow configure plugin by managers
      PLUGIN_FLAG_PROFILING =2,  // allow plugin profiling
      //--- flags borders
      PLUGIN_FLAG_NONE     =0,
      PLUGIN_FLAG_ALL      =PLUGIN_FLAG_MAN_CONFIG|PLUGIN_FLAG_PROFILING
     };
   //--- plugin mode
   enum EnPluginMode
     {
      PLUGIN_DISABLED   =0,
      PLUGIN_ENABLED    =1,
      //--- enumeration borders
      PLUGIN_FIRST      =PLUGIN_DISABLED,
      PLUGIN_LAST       =PLUGIN_ENABLED,
     };
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTConPlugin* param)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- plugin name
   virtual LPCWSTR   Name(void) const=0;
   virtual MTAPIRES  Name(LPCWSTR name)=0;
   //--- MT5 server ID
   virtual UINT64    Server(void) const=0;
   virtual MTAPIRES  Server(const UINT64 server)=0;
   //--- plugin file name
   virtual LPCWSTR   Module(void) const=0;
   virtual MTAPIRES  Module(LPCWSTR name)=0;
   //--- plugin mode
   virtual UINT      Mode(void) const=0;
   virtual MTAPIRES  Mode(const UINT mode)=0;
   //--- plugin parameters
   virtual MTAPIRES  ParameterAdd(IMTConParam* param)=0;
   virtual MTAPIRES  ParameterUpdate(const UINT pos,const IMTConParam* param)=0;
   virtual MTAPIRES  ParameterDelete(const UINT pos)=0;
   virtual MTAPIRES  ParameterClear(void)=0;
   virtual MTAPIRES  ParameterShift(const UINT pos,const int shift)=0;
   virtual UINT      ParameterTotal(void) const=0;
   virtual MTAPIRES  ParameterNext(const UINT pos,IMTConParam* param) const=0;
   virtual MTAPIRES  ParameterGet(LPCWSTR name,IMTConParam* param) const=0;
   //--- EnPluginFlags
   virtual UINT      Flags(void) const=0;
   virtual MTAPIRES  Flags(const UINT flags)=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTConPlugin(void) {}
  };

class IMTConPluginSink
  {
public:
   virtual void      OnPluginAdd(const IMTConPlugin*    /*plugin*/) {  }
   virtual void      OnPluginUpdate(const IMTConPlugin* /*plugin*/) {  }
   virtual void      OnPluginDelete(const IMTConPlugin* /*plugin*/) {  }
   virtual void      OnPluginSync(void)                             {  }
  };

#pragma pack(push,1)
struct MTServerInfo
  {
   wchar_t           platform_name[64];                     // platform name
   wchar_t           platform_owner[128];                   // platform owner
   UINT              server_version;                        // server version
   UINT              server_build;                          // server build
   UINT              server_type;                           // server type
   UINT64            server_id;                             // server id
   UINT              reserved[32];                          // reserved
  };
#pragma pack(pop)

class IMTConHoliday
  {
public:
   //--- holiday modes
   enum EnHolidayMode
     {
      HOLIDAY_DISABLED=0,
      HOLIDAY_ENABLED =1,
      //--- enumeration borders
      HOLIDAY_FIRST   =HOLIDAY_DISABLED,
      HOLIDAY_LAST    =HOLIDAY_ENABLED,
     };
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTConHoliday* holiday)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- holiday description
   virtual LPCWSTR   Description(void) const=0;
   virtual MTAPIRES  Description(LPCWSTR descr)=0;
   //--- EnHolidayMode
   virtual UINT      Mode(void) const=0;
   virtual MTAPIRES  Mode(const UINT mode)=0;
   //--- holiday year (for example: 2010, 0 - every year)
   virtual UINT      Year(void) const=0;
   virtual MTAPIRES  Year(const UINT year)=0;
   //--- holiday month (1-January, 12-December)
   virtual UINT      Month(void) const=0;
   virtual MTAPIRES  Month(const UINT month)=0;
   //--- holiday day (1-31)
   virtual UINT      Day(void) const=0;
   virtual MTAPIRES  Day(const UINT day)=0;
   //--- holiday working time from (in minutes - 100 means 01:40)
   virtual UINT      WorkFrom(void) const=0;
   virtual MTAPIRES  WorkFrom(const UINT from)=0;
   virtual UINT      WorkFromHours(void) const=0;
   virtual UINT      WorkFromMinutes(void) const=0;
   //--- holiday working time to (in minutes - 100 means 01:40)
   virtual UINT      WorkTo(void) const=0;
   virtual MTAPIRES  WorkTo(const UINT from)=0;
   virtual UINT      WorkToHours(void) const=0;
   virtual UINT      WorkToMinutes(void) const=0;
   //--- holiday symbols list
   virtual MTAPIRES  SymbolAdd(LPCWSTR path)=0;
   virtual MTAPIRES  SymbolUpdate(const UINT pos,LPCWSTR path)=0;
   virtual MTAPIRES  SymbolDelete(const UINT pos)=0;
   virtual MTAPIRES  SymbolClear(void)=0;
   virtual MTAPIRES  SymbolShift(const UINT pos,const int shift)=0;
   virtual UINT      SymbolTotal(void) const=0;
   virtual LPCWSTR   SymbolNext(const UINT pos) const=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTConHoliday(void) {}
  };

class IMTConHolidaySink
  {
public:
   virtual void      OnHolidayAdd(const IMTConHoliday*    /*config*/) {  }
   virtual void      OnHolidayUpdate(const IMTConHoliday* /*config*/) {  }
   virtual void      OnHolidayDelete(const IMTConHoliday* /*config*/) {  }
   virtual void      OnHolidaySync(void)                              {  }
  };

class IMTConFirewall
  {
public:
   //--- firewall actions
   enum EnAction
     {
      ACCESS_BLOCK    =0,              // block
      ACCESS_PERMIT   =1,              // permit
      ACCESS_WHITELIST=2,              // permit always
      //--- enumeration borders
      ACCESS_FIRST    =ACCESS_BLOCK,
      ACCESS_LAST     =ACCESS_WHITELIST
     };
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTConFirewall* param)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- action with connection (EnAction)
   virtual UINT      Action(void) const=0;
   virtual MTAPIRES  Action(const UINT action)=0;
   //--- IP range from
   virtual LPCWSTR   From(void) const=0;
   virtual MTAPIRES  From(LPCWSTR name)=0;
   //--- IP range to
   virtual LPCWSTR   To(void) const=0;
   virtual MTAPIRES  To(LPCWSTR value)=0;
   //--- comment
   virtual LPCWSTR   Comment(void) const=0;
   virtual MTAPIRES  Comment(LPCWSTR comment)=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTConFirewall(void) {}
  };

class IMTConFirewallSink
  {
public:
   virtual void      OnFirewallAdd(const IMTConFirewall*    /*config*/) {  }
   virtual void      OnFirewallUpdate(const IMTConFirewall* /*config*/) {  }
   virtual void      OnFirewallDelete(const IMTConFirewall* /*config*/) {  }
   virtual void      OnFirewallSync(void)                               {  }
  };

class IMTConSymbolSession
  {
public:
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTConSymbolSession* symbol)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- session start in minutes (60 = 01:00)
   virtual UINT      Open(void) const=0;
   virtual MTAPIRES  Open(const UINT open)=0;
   //--- session start hours and minutes
   virtual UINT      OpenHours(void) const=0;
   virtual UINT      OpenMinutes(void) const=0;
   //--- session end in minutes (60 = 01:00)
   virtual UINT      Close(void) const=0;
   virtual MTAPIRES  Close(const UINT close)=0;
   //--- session end hours and minutes
   virtual UINT      CloseHours(void) const=0;
   virtual UINT      CloseMinutes(void) const=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTConSymbolSession(void) {}
  };

class IMTConSymbol
  {
public:
   //--- economical sectors
   enum EnSectors
     {
      SECTOR_UNDEFINED              =0,
      SECTOR_BASIC_MATERIALS        =1,
      SECTOR_COMMUNICATION_SERVICES =2,
      SECTOR_CONSUMER_CYCLICAL      =3,
      SECTOR_CONSUMER_DEFENSIVE     =4,
      SECTOR_ENERGY                 =5,
      SECTOR_FINANCIAL              =6,
      SECTOR_HEALTHCARE             =7,
      SECTOR_INDUSTRIALS            =8,
      SECTOR_REAL_ESTATE            =9,
      SECTOR_TECHNOLOGY             =10,
      SECTOR_UTILITIES              =11,
      SECTOR_CURRENCY               =12,
      SECTOR_CURRENCY_CRYPTO        =13,
      SECTOR_INDEXES                =14,
      SECTOR_COMMODITIES            =15,
      //--- enumeration borders
      SECTOR_FIRST                  =SECTOR_UNDEFINED,
      SECTOR_LAST                   =SECTOR_COMMODITIES
     };
   //--- economical industries
   enum EnIndustries
     {
      INDUSTRY_UNDEFINED            =0,
      //--- 
      //--- Basic Materials
      //--- 
      INDUSTRY_AGRICULTURAL_INPUTS  =1,
      INDUSTRY_ALUMINIUM            =2,
      INDUSTRY_BUILDING_MATERIALS   =3,
      INDUSTRY_CHEMICALS            =4,
      INDUSTRY_COKING_COAL          =5,
      INDUSTRY_COPPER               =6,
      INDUSTRY_GOLD                 =7,
      INDUSTRY_LUMBER_WOOD          =8,
      INDUSTRY_INDUSTRIAL_METALS    =9,
      INDUSTRY_PRECIOUS_METALS      =10,
      INDUSTRY_PAPER                =11,
      INDUSTRY_SILVER               =12,
      INDUSTRY_SPECIALTY_CHEMICALS  =13,
      INDUSTRY_STEEL                =14,
      //--- enumeration borders
      INDUSTRY_BASIC_MATERIALS_FIRST=1,
      INDUSTRY_BASIC_MATERIALS_LAST =14,
      INDUSTRY_BASIC_MATERIALS_END  =50,
      //--- 
      //--- Communication Services
      //--- 
      INDUSTRY_ADVERTISING          =51,
      INDUSTRY_BROADCASTING         =52,
      INDUSTRY_GAMING_MULTIMEDIA    =53,
      INDUSTRY_ENTERTAINMENT        =54,
      INDUSTRY_INTERNET_CONTENT     =55,
      INDUSTRY_PUBLISHING           =56,
      INDUSTRY_TELECOM              =57,
      //--- enumeration borders
      INDUSTRY_COMMUNICATION_FIRST  =51,
      INDUSTRY_COMMUNICATION_LAST   =57,
      INDUSTRY_COMMUNICATION_END    =100,
      //--- 
      //--- Consumer Cyclical
      //--- 
      INDUSTRY_APPAREL_MANUFACTURING=101,
      INDUSTRY_APPAREL_RETAIL       =102,
      INDUSTRY_AUTO_MANUFACTURERS   =103,
      INDUSTRY_AUTO_PARTS           =104,
      INDUSTRY_AUTO_DEALERSHIP      =105,
      INDUSTRY_DEPARTMENT_STORES    =106,
      INDUSTRY_FOOTWEAR_ACCESSORIES =107,
      INDUSTRY_FURNISHINGS          =108,
      INDUSTRY_GAMBLING             =109,
      INDUSTRY_HOME_IMPROV_RETAIL   =110,
      INDUSTRY_INTERNET_RETAIL      =111,
      INDUSTRY_LEISURE              =112,
      INDUSTRY_LODGING              =113,
      INDUSTRY_LUXURY_GOODS         =114,
      INDUSTRY_PACKAGING_CONTAINERS =115,
      INDUSTRY_PERSONAL_SERVICES    =116,
      INDUSTRY_RECREATIONAL_VEHICLES=117,
      INDUSTRY_RESIDENT_CONSTRUCTION=118,
      INDUSTRY_RESORTS_CASINOS      =119,
      INDUSTRY_RESTAURANTS          =120,
      INDUSTRY_SPECIALTY_RETAIL     =121,
      INDUSTRY_TEXTILE_MANUFACTURING=122,
      INDUSTRY_TRAVEL_SERVICES      =123,
      //--- enumeration borders
      INDUSTRY_CONSUMER_CYCL_FIRST  =101,
      INDUSTRY_CONSUMER_CYCL_LAST   =123,
      INDUSTRY_CONSUMER_CYCL_END    =150,
      //--- 
      //--- Consumer Defensive
      //--- 
      INDUSTRY_BEVERAGES_BREWERS    =151,
      INDUSTRY_BEVERAGES_NON_ALCO   =152,
      INDUSTRY_BEVERAGES_WINERIES   =153,
      INDUSTRY_CONFECTIONERS        =154,
      INDUSTRY_DISCOUNT_STORES      =155,
      INDUSTRY_EDUCATION_TRAINIG    =156,
      INDUSTRY_FARM_PRODUCTS        =157,
      INDUSTRY_FOOD_DISTRIBUTION    =158,
      INDUSTRY_GROCERY_STORES       =159,
      INDUSTRY_HOUSEHOLD_PRODUCTS   =160,
      INDUSTRY_PACKAGED_FOODS       =161,
      INDUSTRY_TOBACCO              =162,
      //--- enumeration borders
      INDUSTRY_CONSUMER_DEF_FIRST   =151,
      INDUSTRY_CONSUMER_DEF_LAST    =162,
      INDUSTRY_CONSUMER_DEF_END     =200,
      //--- 
      //--- Energy
      //--- 
      INDUSTRY_OIL_GAS_DRILLING     =201,
      INDUSTRY_OIL_GAS_EP           =202,
      INDUSTRY_OIL_GAS_EQUIPMENT    =203,
      INDUSTRY_OIL_GAS_INTEGRATED   =204,
      INDUSTRY_OIL_GAS_MIDSTREAM    =205,
      INDUSTRY_OIL_GAS_REFINING     =206,
      INDUSTRY_THERMAL_COAL         =207,
      INDUSTRY_URANIUM              =208,
      //--- enumeration borders
      INDUSTRY_ENERGY_FIRST         =201,
      INDUSTRY_ENERGY_LAST          =208,
      INDUSTRY_ENERGY_END           =250,
      //--- 
      //--- Financial
      //--- 
      INDUSTRY_EXCHANGE_TRADED_FUND   =251,
      INDUSTRY_ASSETS_MANAGEMENT      =252,
      INDUSTRY_BANKS_DIVERSIFIED      =253,
      INDUSTRY_BANKS_REGIONAL         =254,
      INDUSTRY_CAPITAL_MARKETS        =255,
      INDUSTRY_CLOSE_END_FUND_DEBT    =256,
      INDUSTRY_CLOSE_END_FUND_EQUITY  =257,
      INDUSTRY_CLOSE_END_FUND_FOREIGN =258,
      INDUSTRY_CREDIT_SERVICES        =259,
      INDUSTRY_FINANCIAL_CONGLOMERATE =260,
      INDUSTRY_FINANCIAL_DATA_EXCHANGE=261,
      INDUSTRY_INSURANCE_BROKERS      =262,
      INDUSTRY_INSURANCE_DIVERSIFIED  =263,
      INDUSTRY_INSURANCE_LIFE         =264,
      INDUSTRY_INSURANCE_PROPERTY     =265,
      INDUSTRY_INSURANCE_REINSURANCE  =266,
      INDUSTRY_INSURANCE_SPECIALTY    =267,
      INDUSTRY_MORTGAGE_FINANCE       =268,
      INDUSTRY_SHELL_COMPANIES        =269,
      //--- enumeration borders
      INDUSTRY_FINANCIAL_FIRST        =251,
      INDUSTRY_FINANCIAL_LAST         =269,
      INDUSTRY_FINANCIAL_END          =300,
      //--- 
      //--- Healthcare
      //--- 
      INDUSTRY_BIOTECHNOLOGY        =301,
      INDUSTRY_DIAGNOSTICS_RESEARCH =302,
      INDUSTRY_DRUGS_MANUFACTURERS  =303,
      INDUSTRY_DRUGS_MANUFACTURERS_SPEC=304,
      INDUSTRY_HEALTHCARE_PLANS        =305,
      INDUSTRY_HEALTH_INFORMATION   =306,
      INDUSTRY_MEDICAL_FACILITIES   =307,
      INDUSTRY_MEDICAL_DEVICES      =308,
      INDUSTRY_MEDICAL_DISTRIBUTION =309,
      INDUSTRY_MEDICAL_INSTRUMENTS  =310,
      INDUSTRY_PHARM_RETAILERS      =311,
      //--- enumeration borders
      INDUSTRY_HEALTHCARE_FIRST     =301,
      INDUSTRY_HEALTHCARE_LAST      =311,
      INDUSTRY_HEALTHCARE_END       =350,
      //--- 
      //--- Industrials
      //--- 
      INDUSTRY_AEROSPACE_DEFENSE    =351,
      INDUSTRY_AIRLINES             =352,
      INDUSTRY_AIRPORTS_SERVICES    =353,
      INDUSTRY_BUILDING_PRODUCTS    =354,
      INDUSTRY_BUSINESS_EQUIPMENT   =355,
      INDUSTRY_CONGLOMERATES        =356,
      INDUSTRY_CONSULTING_SERVICES  =357,
      INDUSTRY_ELECTRICAL_EQUIPMENT =358,
      INDUSTRY_ENGINEERING_CONSTRUCTION  =359,
      INDUSTRY_FARM_HEAVY_MACHINERY      =360,
      INDUSTRY_INDUSTRIAL_DISTRIBUTION   =361,
      INDUSTRY_INFRASTRUCTURE_OPERATIONS =362,
      INDUSTRY_FREIGHT_LOGISTICS    =363,
      INDUSTRY_MARINE_SHIPPING      =364,
      INDUSTRY_METAL_FABRICATION    =365,
      INDUSTRY_POLLUTION_CONTROL    =366,
      INDUSTRY_RAILROADS            =367,
      INDUSTRY_RENTAL_LEASING       =368,
      INDUSTRY_SECURITY_PROTECTION  =369,
      INDUSTRY_SPEALITY_BUSINESS_SERVICES=370,
      INDUSTRY_SPEALITY_MACHINERY   =371,
      INDUSTRY_STUFFING_EMPLOYMENT  =372,
      INDUSTRY_TOOLS_ACCESSORIES    =373,
      INDUSTRY_TRUCKING             =374,
      INDUSTRY_WASTE_MANAGEMENT     =375,
      //--- enumeration borders
      INDUSTRY_INDUSTRIALS_FIRST    =351,
      INDUSTRY_INDUSTRIALS_LAST     =375,
      INDUSTRY_INDUSTRIALS_END      =400,
      //--- 
      //--- Real Estate
      //--- 
      INDUSTRY_REAL_ESTATE_DEVELOPMENT=401,
      INDUSTRY_REAL_ESTATE_DIVERSIFIED=402,
      INDUSTRY_REAL_ESTATE_SERVICES   =403,
      INDUSTRY_REIT_DIVERSIFIED     =404,
      INDUSTRY_REIT_HEALTCARE       =405,
      INDUSTRY_REIT_HOTEL_MOTEL     =406,
      INDUSTRY_REIT_INDUSTRIAL      =407,
      INDUSTRY_REIT_MORTAGE         =408,
      INDUSTRY_REIT_OFFICE          =409,
      INDUSTRY_REIT_RESIDENTAL      =410,
      INDUSTRY_REIT_RETAIL          =411,
      INDUSTRY_REIT_SPECIALITY      =412,
      //--- enumeration borders
      INDUSTRY_REAL_ESTATE_FIRST    =401,
      INDUSTRY_REAL_ESTATE_LAST     =412,
      INDUSTRY_REAL_ESTATE_END      =450,
      //--- 
      //--- Technology
      //--- 
      INDUSTRY_COMMUNICATION_EQUIPMENT=451,
      INDUSTRY_COMPUTER_HARDWARE      =452,
      INDUSTRY_CONSUMER_ELECTRONICS   =453,
      INDUSTRY_ELECTRONIC_COMPONENTS  =454,
      INDUSTRY_ELECTRONIC_DISTRIBUTION=455,
      INDUSTRY_IT_SERVICES            =456,
      INDUSTRY_SCIENTIFIC_INSTRUMENTS =457,
      INDUSTRY_SEMICONDUCTOR_EQUIPMENT=458,
      INDUSTRY_SEMICONDUCTORS         =459,
      INDUSTRY_SOFTWARE_APPLICATION   =460,
      INDUSTRY_SOFTWARE_INFRASTRUCTURE=461,
      INDUSTRY_SOLAR                  =462,
      //--- enumeration borders
      INDUSTRY_TECHNOLOGY_FIRST       =451,
      INDUSTRY_TECHNOLOGY_LAST        =462,
      INDUSTRY_TECHNOLOGY_END         =500,
      //--- 
      //--- Utilities
      //--- 
      INDUSTRY_UTILITIES_DIVERSIFIED       =501,
      INDUSTRY_UTILITIES_POWERPRODUCERS    =502,
      INDUSTRY_UTILITIES_RENEWABLE         =503,
      INDUSTRY_UTILITIES_REGULATED_ELECTRIC=504,
      INDUSTRY_UTILITIES_REGULATED_GAS     =505,
      INDUSTRY_UTILITIES_REGULATED_WATER   =506,
      //--- enumeration borders
      INDUSTRY_UTILITIES_FIRST        =501,
      INDUSTRY_UTILITIES_LAST         =506,
      INDUSTRY_UTILITIES_END          =550,
      //--- 
      //--- Commodities
      //--- 
      INDUSTRY_COMMODITIES_AGRICULTURAL=551,
      INDUSTRY_COMMODITIES_ENERGY     =552,
      INDUSTRY_COMMODITIES_METALS     =553,
      INDUSTRY_COMMODITIES_PRECIOUS   =554,
      //--- enumeration borders
      INDUSTRY_COMMODITIES_FIRST      =551,
      INDUSTRY_COMMODITIES_LAST       =554,
      INDUSTRY_COMMODITIES_END        =600,
      //--- enumeration borders
      INDUSTRY_FIRST                  =0,
      INDUSTRY_LAST                   =INDUSTRY_COMMODITIES_LAST
     };
   //--- allowed filling modes flags
   enum EnFillingFlags
     {
      FILL_FLAGS_NONE         =0, // none
      FILL_FLAGS_FOK          =1, // allowed FOK
      FILL_FLAGS_IOC          =2, // allowed IOC
      //--- flags borders
      FILL_FLAGS_FIRST        =FILL_FLAGS_FOK,
      FILL_FLAGS_ALL          =FILL_FLAGS_FOK|FILL_FLAGS_IOC
     };
   //--- allowed order expiration modes flags
   enum EnExpirationFlags
     {
      TIME_FLAGS_NONE         =0, // none
      TIME_FLAGS_GTC          =1, // allowed Good Till Cancel
      TIME_FLAGS_DAY          =2, // allowed Good Till Day
      TIME_FLAGS_SPECIFIED    =4, // allowed specified expiration date
      TIME_FLAGS_SPECIFIED_DAY=8, // allowed specified expiration date as day
      //--- flags borders
      TIME_FLAGS_FIRST        =TIME_FLAGS_GTC,
      TIME_FLAGS_ALL          =TIME_FLAGS_GTC|TIME_FLAGS_DAY|TIME_FLAGS_SPECIFIED|TIME_FLAGS_SPECIFIED_DAY
     };
   //--- allowed order flags
   enum EnOrderFlags
     {
      ORDER_FLAGS_NONE        =0,  // none
      ORDER_FLAGS_MARKET      =1,  // market orders
      ORDER_FLAGS_LIMIT       =2,  // limit orders
      ORDER_FLAGS_STOP        =4,  // stop orders
      ORDER_FLAGS_STOP_LIMIT  =8,  // stop limit orders
      ORDER_FLAGS_SL          =16, // sl orders
      ORDER_FLAGS_TP          =32, // tp orders
      ORDER_FLAGS_CLOSEBY     =64, // close-by orders
      //--- flags borders
      ORDER_FLAGS_FIRST       =ORDER_FLAGS_MARKET,
      ORDER_FLAGS_ALL         =ORDER_FLAGS_MARKET|ORDER_FLAGS_LIMIT|ORDER_FLAGS_STOP|ORDER_FLAGS_STOP_LIMIT|ORDER_FLAGS_SL|ORDER_FLAGS_TP|ORDER_FLAGS_CLOSEBY
     };
   //--- allowed trade modes
   enum EnTradeMode
     {
      TRADE_DISABLED          =0, // trade disabled
      TRADE_LONGONLY          =1, // only long positions allowed
      TRADE_SHORTONLY         =2, // only short positions allowed
      TRADE_CLOSEONLY         =3, // only position closure
      TRADE_FULL              =4, // all trade operations are allowed
      //--- enumeration borders
      TRADE_FIRST             =TRADE_DISABLED,
      TRADE_LAST              =TRADE_FULL
     };
   //--- order execution modes
   enum EnExecutionMode
     {
      EXECUTION_REQUEST       =0, // Request Execution
      EXECUTION_INSTANT       =1, // Instant Execution
      EXECUTION_MARKET        =2, // Market Execution
      EXECUTION_EXCHANGE      =3, // Exchange Execution
      //--- enumeration borders
      EXECUTION_FIRST         =EXECUTION_REQUEST,
      EXECUTION_LAST          =EXECUTION_EXCHANGE
     };
   //--- profit and margin calculation modes
   enum EnCalcMode
     {
      //--- market maker modes
      TRADE_MODE_FOREX              =0,
      TRADE_MODE_FUTURES            =1,
      TRADE_MODE_CFD                =2,
      TRADE_MODE_CFDINDEX           =3,
      TRADE_MODE_CFDLEVERAGE        =4,
      TRADE_MODE_FOREX_NO_LEVERAGE  =5,
      //--- market makers enumerations
      TRADE_MODE_MM_FIRST           =TRADE_MODE_FOREX,
      TRADE_MODE_MM_LAST            =TRADE_MODE_FOREX_NO_LEVERAGE,
      //--- exchange modes
      TRADE_MODE_EXCH_STOCKS        =32,
      TRADE_MODE_EXCH_FUTURES       =33,
      TRADE_MODE_EXCH_FUTURES_FORTS =34,
      TRADE_MODE_EXCH_OPTIONS       =35,
      TRADE_MODE_EXCH_OPTIONS_MARGIN=36,
      TRADE_MODE_EXCH_BONDS         =37,
      TRADE_MODE_EXCH_STOCKS_MOEX   =38,
      TRADE_MODE_EXCH_BONDS_MOEX    =39,
      //--- exchange enumerations
      TRADE_MODE_EXCH_FIRST         =TRADE_MODE_EXCH_STOCKS,
      TRADE_MODE_EXCH_LAST          =TRADE_MODE_EXCH_BONDS_MOEX,
      //--- service modes
      TRADE_MODE_SERV_COLLATERAL    =64,
      //--- service enumerations
      TRADE_MODE_SERV_FIRST         =TRADE_MODE_SERV_COLLATERAL,
      TRADE_MODE_SERV_LAST          =TRADE_MODE_SERV_COLLATERAL,
      //--- enumeration borders
      TRADE_MODE_FIRST              =TRADE_MODE_FOREX,
      TRADE_MODE_LAST               =TRADE_MODE_SERV_COLLATERAL
     };
   //--- orders expiration modes
   enum EnGTCMode
     {
      ORDERS_GTC              =0,
      ORDERS_DAILY            =1,
      ORDERS_DAILY_NO_STOPS   =2,
      //--- enumeration borders
      ORDERS_FIRST            =ORDERS_GTC,
      ORDERS_LAST             =ORDERS_DAILY_NO_STOPS
     };
   //--- tick collection flags
   enum EnTickFlags
     {
      TICK_REALTIME           =1,  // allow realtime tick apply
      TICK_COLLECTRAW         =2,  // allow to collect raw ticks
      TICK_FEED_STATS         =4,  // allow to receive price statisticks from datafeeds
      TICK_NEGATIVE_PRICES    =8,  // allow to receive negative prices
      //--- flags borders
      TICK_NONE               =0,
      TICK_ALL                =TICK_REALTIME|TICK_COLLECTRAW|TICK_FEED_STATS|TICK_NEGATIVE_PRICES
     };
   //--- chart mode
   enum EnChartMode
     {
      CHART_MODE_BID_PRICE    =0,
      CHART_MODE_LAST_PRICE   =1,
      CHART_MODE_OLD          =255,
      //--- enumeration borders
      CHART_MODE_FIRST        =CHART_MODE_BID_PRICE,
      CHART_MODE_LAST         =CHART_MODE_OLD
     };
   //--- margin check modes
   enum EnMarginFlags
     {
      MARGIN_FLAGS_NONE             =0x0000000,  // none
      MARGIN_FLAGS_CHECK_PROCESS    =0x0000001,  // check margin after dealer confirmation
      MARGIN_FLAGS_CHECK_SLTP       =0x0000002,  // check margin on SL-TP trigger
      MARGIN_FLAGS_HEDGE_LARGE_LEG  =0x0000004,  // check margin for hedged positions using large leg
      MARGIN_FLAGS_EXCLUDE_PL       =0x0000008,  // exclude buy positions PL from free margin and margin level calculation
      //--- flags borders
      MARGIN_FLAGS_ALL              =MARGIN_FLAGS_CHECK_PROCESS|MARGIN_FLAGS_CHECK_SLTP|MARGIN_FLAGS_HEDGE_LARGE_LEG|MARGIN_FLAGS_EXCLUDE_PL
     };
   //--- swaps calculation modes
   enum EnSwapMode
     {
      SWAP_DISABLED           =0,
      SWAP_BY_POINTS          =1,
      SWAP_BY_SYMBOL_CURRENCY =2,
      SWAP_BY_MARGIN_CURRENCY =3,
      SWAP_BY_GROUP_CURRENCY  =4,
      SWAP_BY_INTEREST_CURRENT=5,
      SWAP_BY_INTEREST_OPEN   =6,
      SWAP_REOPEN_BY_CLOSE_PRICE=7,
      SWAP_REOPEN_BY_BID        =8,
      SWAP_BY_PROFIT_CURRENCY   =9,
      //--- enumeration borders
      SWAP_FIRST              =SWAP_DISABLED,
      SWAP_LAST               =SWAP_BY_PROFIT_CURRENCY
     };
   //--- swaps days
   enum EnSwapDays
     {
      SWAP_DAY_SUNDAY         =0,
      SWAP_DAY_MONDAY         =1,
      SWAP_DAY_TUESDAY        =2,
      SWAP_DAY_WEDNESDAY      =3,
      SWAP_DAY_THURSDAY       =4,
      SWAP_DAY_FRIDAY         =5,
      SWAP_DAY_SATURDAY       =6,
      SWAP_DAY_DISABLED       =7,
      //--- enumeration borders
      SWAP_DAY_FIRST          =SWAP_DAY_SUNDAY,
      SWAP_DAY_LAST           =SWAP_DAY_DISABLED
     };
   //--- swap flags
   enum EnSwapFlags
     {
      SWAP_FLAGS_NONE             =0x00000000,
      SWAP_FLAGS_CONSIDER_HOLIDAYS=0x00000001,
      //--- enumeration borders
      SWAP_FLAGS_DEFAULT          =SWAP_FLAGS_NONE,
      SWAP_FLAGS_ALL              =SWAP_FLAGS_CONSIDER_HOLIDAYS
     };
   //--- Instant Execution Flags
   enum EnInstantFlags
     {
      INSTANT_FLAGS_NONE             =0x00000000,
      INSTANT_FLAGS_FAST_CONFIRMATION=0x00000001,
      //--- enumeration borders
      INSTANT_FLAGS_ALL              =INSTANT_FLAGS_FAST_CONFIRMATION
     };
   //--- Instant Execution Modes
   enum EnInstantMode
     {
      INSTANT_CHECK_NORMAL    =0,
      //--- enumeration borders
      INSTANT_CHECK_FIRST     =INSTANT_CHECK_NORMAL,
      INSTANT_CHECK_LAST      =INSTANT_CHECK_NORMAL
     };
   //--- Request Execution Flags
   enum EnRequestFlags
     {
      REQUEST_FLAGS_NONE      =0,  // none
      REQUEST_FLAGS_ORDER     =1,  // trade orders should be additional confirmed after quotation
      //--- flags borders
      REQUEST_FLAGS_ALL       =REQUEST_FLAGS_ORDER
     };
   //--- Common Trade Flags
   enum EnTradeFlags
     {
      TRADE_FLAGS_NONE            =0,   // none
      TRADE_FLAGS_PROFIT_BY_MARKET=1,   // convert fx profit using market prices
      TRADE_FLAGS_ALLOW_SIGNALS   =2,   // allow trade signals for symbol
      //--- flags borders
      TRADE_FLAGS_ALL             =TRADE_FLAGS_PROFIT_BY_MARKET|TRADE_FLAGS_ALLOW_SIGNALS,
      TRADE_FLAGS_DEFAULT         =TRADE_FLAGS_ALLOW_SIGNALS
     };
   //--- Margin Rate Types
   enum EnMarginRateTypes
     {
      MARGIN_RATE_BUY         =0,
      MARGIN_RATE_SELL        =1,
      MARGIN_RATE_BUY_LIMIT   =2,
      MARGIN_RATE_SELL_LIMIT  =3,
      MARGIN_RATE_BUY_STOP    =4,
      MARGIN_RATE_SELL_STOP   =5,
      MARGIN_RATE_BUY_STOP_LIMIT =6,
      MARGIN_RATE_SELL_STOP_LIMIT=7,
      //--- enumeration borders
      MARGIN_RATE_FIRST       =MARGIN_RATE_BUY,
      MARGIN_RATE_LAST        =MARGIN_RATE_SELL_STOP_LIMIT
     };
   //--- Options Mode
   enum EnOptionMode
     {
      OPTION_MODE_EUROPEAN_CALL=0,
      OPTION_MODE_EUROPEAN_PUT =1,
      OPTION_MODE_AMERICAN_CALL=2,
      OPTION_MODE_AMERICAN_PUT =3,
      //--- enumeration borders
      OPTION_MODE_FIRST        =OPTION_MODE_EUROPEAN_CALL,
      OPTION_MODE_LAST         =OPTION_MODE_AMERICAN_PUT
     };
   //--- Splice Type
   enum EnSpliceType
     {
      SPLICE_NONE              =0,
      SPLICE_UNADJUSTED        =1,
      SPLICE_ADJUSTED          =2,
      //--- enumeration borders
      SPLICE_FIRST             =SPLICE_NONE,
      SPLICE_LAST              =SPLICE_ADJUSTED
     };
   //--- Splice Time Type
   enum EnSpliceTimeType
     {
      SPLICE_TIME_EXPIRATION   =0,
      //--- enumeration borders
      SPLICE_TIME_FIRST        =SPLICE_TIME_EXPIRATION,
      SPLICE_TIME_LAST         =SPLICE_TIME_EXPIRATION
     };
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTConSymbol* symbol)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- name
   virtual LPCWSTR   Symbol(void) const=0;
   virtual MTAPIRES  Symbol(LPCWSTR symbol)=0;
   //--- hierarchical symbol path (including symbol name)
   virtual LPCWSTR   Path(void) const=0;
   virtual MTAPIRES  Path(LPCWSTR path)=0;
   //--- ISIN
   virtual LPCWSTR   ISIN(void) const=0;
   virtual MTAPIRES  ISIN(LPCWSTR isin)=0;
   //--- local description
   virtual LPCWSTR   Description(void) const=0;
   virtual MTAPIRES  Description(LPCWSTR descr)=0;
   //--- internation description
   virtual LPCWSTR   International(void) const=0;
   virtual MTAPIRES  International(LPCWSTR intern)=0;
   //--- basic symbol name
   virtual LPCWSTR   Basis(void) const=0;
   virtual MTAPIRES  Basis(LPCWSTR basis)=0;
   //--- source symbol name
   virtual LPCWSTR   Source(void) const=0;
   virtual MTAPIRES  Source(LPCWSTR source)=0;
   //--- symbol specification page URL
   virtual LPCWSTR   Page(void) const=0;
   virtual MTAPIRES  Page(LPCWSTR page)=0;
   //--- symbol base currency
   virtual LPCWSTR   CurrencyBase(void) const=0;
   virtual MTAPIRES  CurrencyBase(LPCWSTR currency)=0;
   virtual UINT      CurrencyBaseDigits(void) const=0;
   //--- symbol profit currency
   virtual LPCWSTR   CurrencyProfit(void) const=0;
   virtual MTAPIRES  CurrencyProfit(LPCWSTR currency)=0;
   virtual UINT      CurrencyProfitDigits(void) const=0;
   //--- symbol margin currency
   virtual LPCWSTR   CurrencyMargin(void) const=0;
   virtual MTAPIRES  CurrencyMargin(LPCWSTR currency)=0;
   virtual UINT      CurrencyMarginDigits(void) const=0;
   //--- symbol color
   virtual COLORREF  Color(void) const=0;
   virtual MTAPIRES  Color(const COLORREF color)=0;
   //--- symbol background color
   virtual COLORREF  ColorBackground(void) const=0;
   virtual MTAPIRES  ColorBackground(const COLORREF color)=0;
   //--- symbol digits
   virtual UINT      Digits(void) const=0;
   virtual MTAPIRES  Digits(const UINT digits)=0;
   //--- symbol digits derivation (1/10^digits & 10^digits)
   virtual double    Point(void) const=0;
   virtual double    Multiply(void) const=0;
   //--- EnTickFlags
   virtual UINT64    TickFlags(void) const=0;
   virtual MTAPIRES  TickFlags(const UINT64 flags)=0;
   //--- Depth of Market depth (both legs)
   virtual UINT      TickBookDepth(void) const=0;
   virtual MTAPIRES  TickBookDepth(const UINT depth)=0;
   //--- filtration soft level
   virtual UINT      FilterSoft(void) const=0;
   virtual MTAPIRES  FilterSoft(const UINT filter)=0;
   //--- filtration soft level counter
   virtual UINT      FilterSoftTicks(void) const=0;
   virtual MTAPIRES  FilterSoftTicks(const UINT ticks)=0;
   //--- filtration hard level
   virtual UINT      FilterHard(void) const=0;
   virtual MTAPIRES  FilterHard(const UINT filter)=0;
   //--- filtration hard level counter
   virtual UINT      FilterHardTicks(void) const=0;
   virtual MTAPIRES  FilterHardTicks(const UINT ticks)=0;
   //--- filtration discard level
   virtual UINT      FilterDiscard(void) const=0;
   virtual MTAPIRES  FilterDiscard(const UINT ticks)=0;
   //--- spread max value
   virtual UINT      FilterSpreadMax(void) const=0;
   virtual MTAPIRES  FilterSpreadMax(const UINT spread)=0;
   //--- spread min value
   virtual UINT      FilterSpreadMin(void) const=0;
   virtual MTAPIRES  FilterSpreadMin(const UINT spread)=0;
   //--- EnTradeMode
   virtual UINT      TradeMode(void) const=0;
   virtual MTAPIRES  TradeMode(const UINT mode)=0;
   //--- EnCalcMode
   virtual UINT      CalcMode(void) const=0;
   virtual MTAPIRES  CalcMode(const UINT mode)=0;
   //--- EnExecutionMode
   virtual UINT      ExecMode(void) const=0;
   virtual MTAPIRES  ExecMode(const UINT mode)=0;
   //--- EnGTCMode
   virtual UINT      GTCMode(void) const=0;
   virtual MTAPIRES  GTCMode(const UINT mode)=0;
   //--- EnFillingFlags
   virtual UINT      FillFlags(void) const=0;
   virtual MTAPIRES  FillFlags(const UINT flags)=0;
   //--- EnExpirationFlags
   virtual UINT      ExpirFlags(void) const=0;
   virtual MTAPIRES  ExpirFlags(const UINT flags)=0;
   //--- symbol spread (0-floating)
   virtual UINT      Spread(void) const=0;
   virtual MTAPIRES  Spread(const UINT spread)=0;
   //--- spread balance
   virtual INT       SpreadBalance(void) const=0;
   virtual MTAPIRES  SpreadBalance(const INT spread)=0;
   //--- spread difference
   virtual INT       SpreadDiff(void) const=0;
   virtual MTAPIRES  SpreadDiff(const INT diff)=0;
   //--- spread difference balance
   virtual INT       SpreadDiffBalance(void) const=0;
   virtual MTAPIRES  SpreadDiffBalance(const INT spread)=0;
   //--- tick value
   virtual double    TickValue(void) const=0;
   virtual MTAPIRES  TickValue(const double value)=0;
   //--- tick size
   virtual double    TickSize(void) const=0;
   virtual MTAPIRES  TickSize(const double size)=0;
   //--- contract size
   virtual double    ContractSize(void) const=0;
   virtual MTAPIRES  ContractSize(const double size)=0;
   //--- stops level
   virtual INT       StopsLevel(void) const=0;
   virtual MTAPIRES  StopsLevel(const INT level)=0;
   //--- freeze level
   virtual INT       FreezeLevel(void) const=0;
   virtual MTAPIRES  FreezeLevel(const INT level)=0;
   //--- quotes timeout 
   virtual UINT      QuotesTimeout(void) const=0;
   virtual MTAPIRES  QuotesTimeout(const UINT timeout)=0;
   //--- minimal volume
   virtual UINT64    VolumeMin(void) const=0;
   virtual MTAPIRES  VolumeMin(const UINT64 volume)=0;
   //--- maximal volume
   virtual UINT64    VolumeMax(void) const=0;
   virtual MTAPIRES  VolumeMax(const UINT64 volume)=0;
   //--- volume step
   virtual UINT64    VolumeStep(void) const=0;
   virtual MTAPIRES  VolumeStep(const UINT64 volume)=0;
   //--- cumulative positions and orders limit
   virtual UINT64    VolumeLimit(void) const=0;
   virtual MTAPIRES  VolumeLimit(const UINT64 volume)=0;
   //--- EnMarginFlags
   virtual UINT      MarginFlags(void) const=0;
   virtual MTAPIRES  MarginFlags(const UINT mode)=0;
   //--- initial margin
   virtual double    MarginInitial(void) const=0;
   virtual MTAPIRES  MarginInitial(const double margin)=0;
   //--- maintenance margin
   virtual double    MarginMaintenance(void) const=0;
   virtual MTAPIRES  MarginMaintenance(const double margin)=0;
   //--- long orders and positions margin rate
   virtual double    MarginLong(void) const=0;
   virtual MTAPIRES  MarginLong(const double margin)=0;
   //--- short orders and positions margin rate
   virtual double    MarginShort(void) const=0;
   virtual MTAPIRES  MarginShort(const double margin)=0;
   //--- limit orders and positions margin rate
   virtual double    MarginLimit(void) const=0;
   virtual MTAPIRES  MarginLimit(const double margin)=0;
   //--- stop orders and positions margin rate
   virtual double    MarginStop(void) const=0;
   virtual MTAPIRES  MarginStop(const double margin)=0;
   //--- stop-limit orders and positions margin rate
   virtual double    MarginStopLimit(void) const=0;
   virtual MTAPIRES  MarginStopLimit(const double margin)=0;
   //--- EnSwapMode
   virtual UINT      SwapMode(void) const=0;
   virtual MTAPIRES  SwapMode(const UINT mode)=0;
   //--- long positions swaps rate
   virtual double    SwapLong(void) const=0;
   virtual MTAPIRES  SwapLong(const double swap)=0;
   //--- short positions swaps rate
   virtual double    SwapShort(void) const=0;
   virtual MTAPIRES  SwapShort(const double swap)=0;
   //--- 3 time swaps day, EnSwapDay
   virtual UINT      Swap3Day(void) const=0;
   virtual MTAPIRES  Swap3Day(const UINT day)=0;
   //--- trade start date
   virtual INT64     TimeStart(void) const=0;
   virtual MTAPIRES  TimeStart(const INT64 start)=0;
   //--- trade end date
   virtual INT64     TimeExpiration(void) const=0;
   virtual MTAPIRES  TimeExpiration(const INT64 expiration)=0;
   //--- quote sessions
   virtual MTAPIRES  SessionQuoteAdd(const UINT wday,IMTConSymbolSession* symbol)=0;
   virtual MTAPIRES  SessionQuoteUpdate(const UINT wday,const UINT pos,const IMTConSymbolSession* session)=0;
   virtual MTAPIRES  SessionQuoteDelete(const UINT wday,const UINT pos)=0;
   virtual MTAPIRES  SessionQuoteClear(const UINT wday)=0;
   virtual MTAPIRES  SessionQuoteShift(const UINT wday,const UINT pos,const int shift)=0;
   virtual UINT      SessionQuoteTotal(const UINT wday) const=0;
   virtual MTAPIRES  SessionQuoteNext(const UINT wday,const UINT pos,IMTConSymbolSession* session) const=0;
   //--- trade sessions
   virtual MTAPIRES  SessionTradeAdd(const UINT wday,IMTConSymbolSession* symbol)=0;
   virtual MTAPIRES  SessionTradeUpdate(const UINT wday,const UINT pos,const IMTConSymbolSession* session)=0;
   virtual MTAPIRES  SessionTradeDelete(const UINT wday,const UINT pos)=0;
   virtual MTAPIRES  SessionTradeClear(const UINT wday)=0;
   virtual MTAPIRES  SessionTradeShift(const UINT wday,const UINT pos,const int shift)=0;
   virtual UINT      SessionTradeTotal(const UINT wday) const=0;
   virtual MTAPIRES  SessionTradeNext(const UINT wday,const UINT pos,IMTConSymbolSession* session) const=0;
   //--- request execution flags
   virtual UINT      REFlags(void) const=0;
   virtual MTAPIRES  REFlags(const UINT flags)=0;
   //--- request execution timeout
   virtual UINT      RETimeout(void) const=0;
   virtual MTAPIRES  RETimeout(const UINT timeout)=0;
   //--- instant execution check mode 
   virtual UINT      IECheckMode(void) const=0;
   virtual MTAPIRES  IECheckMode(const UINT mode)=0;
   //--- instant execution timeout
   virtual UINT      IETimeout(void) const=0;
   virtual MTAPIRES  IETimeout(const UINT timeout)=0;
   //--- instant execution profit slippage
   virtual UINT      IESlipProfit(void) const=0;
   virtual MTAPIRES  IESlipProfit(const UINT slippage)=0;
   //--- instant execution losing slippage
   virtual UINT      IESlipLosing(void) const=0;
   virtual MTAPIRES  IESlipLosing(const UINT slippage)=0;
   //--- instant execution max volume
   virtual UINT64    IEVolumeMax(void) const=0;
   virtual MTAPIRES  IEVolumeMax(const UINT64 volume)=0;
   //--- settle price (for futures)
   virtual double    PriceSettle(void) const=0;
   virtual MTAPIRES  PriceSettle(const double price)=0;
   //--- price limit max (for futures)
   virtual double    PriceLimitMax(void) const=0;
   virtual MTAPIRES  PriceLimitMax(const double price)=0;
   //--- price limit min (for futures)
   virtual double    PriceLimitMin(void) const=0;
   virtual MTAPIRES  PriceLimitMin(const double price)=0;
   //--- EnTradeFlags
   virtual UINT64    TradeFlags(void) const=0;
   virtual MTAPIRES  TradeFlags(const UINT64 flags)=0;
   //--- EnOrderFlags
   virtual UINT      OrderFlags(void) const=0;
   virtual MTAPIRES  OrderFlags(const UINT flags)=0;
   //--- orders and positions margin rates
   virtual double    MarginRateInitial(const UINT type) const=0;
   virtual MTAPIRES  MarginRateInitial(const UINT type,const double margin_rate)=0;
   //--- orders and positions margin rates
   virtual double    MarginRateMaintenance(const UINT type) const=0;
   virtual MTAPIRES  MarginRateMaintenance(const UINT type,const double margin_rate)=0;
   //--- options mode EnOptionMode
   virtual UINT      OptionsMode(void) const=0;
   virtual MTAPIRES  OptionsMode(const UINT mode)=0;
   //--- option strike price value
   virtual double    PriceStrike(void) const=0;
   virtual MTAPIRES  PriceStrike(const double price)=0;
   //--- liquidity rate
   virtual double    MarginRateLiquidity(void) const=0;
   virtual MTAPIRES  MarginRateLiquidity(const double margin_rate)=0;
   //--- bond face value
   virtual double    FaceValue(void) const=0;
   virtual MTAPIRES  FaceValue(const double value)=0;
   //--- bond accrued interest
   virtual double    AccruedInterest(void) const=0;
   virtual MTAPIRES  AccruedInterest(const double interest)=0;
   //--- futures splice type EnSpliceType
   virtual UINT      SpliceType(void) const=0;
   virtual MTAPIRES  SpliceType(const UINT type)=0;
   //--- futures splice time type EnSpliceType
   virtual UINT      SpliceTimeType(void) const=0;
   virtual MTAPIRES  SpliceTimeType(const UINT time_type)=0;
   //--- options splice point in days
   virtual UINT      SpliceTimeDays(void) const=0;
   virtual MTAPIRES  SpliceTimeDays(const UINT days)=0;
   //--- hedged positions margin rate
   virtual double    MarginHedged(void) const=0;
   virtual MTAPIRES  MarginHedged(const double margin)=0;
   //--- currency rate
   virtual double    MarginRateCurrency(void) const=0;
   virtual MTAPIRES  MarginRateCurrency(const double margin_rate)=0;
   //--- gap level
   virtual UINT      FilterGap(void) const=0;
   virtual MTAPIRES  FilterGap(const UINT gap)=0;
   //--- gap level ticks
   virtual UINT      FilterGapTicks(void) const=0;
   virtual MTAPIRES  FilterGapTicks(const UINT ticks)=0;
   //--- chart mode
   virtual UINT      ChartMode(void) const=0;
   virtual MTAPIRES  ChartMode(const UINT mode)=0;
   //---  currency digits
   virtual MTAPIRES  CurrencyBaseDigitsSet(const UINT digits)=0;
   virtual MTAPIRES  CurrencyProfitDigitsSet(const UINT digits)=0;
   virtual MTAPIRES  CurrencyMarginDigitsSet(const UINT digits)=0;
   //--- instant execution flags with extended accuracy
   virtual UINT      IEFlags(void) const=0;
   virtual MTAPIRES  IEFlags(const UINT flags)=0;
   //--- minimal volume with extended accuracy
   virtual UINT64    VolumeMinExt(void) const=0;
   virtual MTAPIRES  VolumeMinExt(const UINT64 volume)=0;
   //--- maximal volume with extended accuracy
   virtual UINT64    VolumeMaxExt(void) const=0;
   virtual MTAPIRES  VolumeMaxExt(const UINT64 volume)=0;
   //--- volume step with extended accuracy
   virtual UINT64    VolumeStepExt(void) const=0;
   virtual MTAPIRES  VolumeStepExt(const UINT64 volume)=0;
   //--- cumulative positions and orders limit with extended accuracy
   virtual UINT64    VolumeLimitExt(void) const=0;
   virtual MTAPIRES  VolumeLimitExt(const UINT64 volume)=0;
   //--- instant execution max volume with extended accuracy
   virtual UINT64    IEVolumeMaxExt(void) const=0;
   virtual MTAPIRES  IEVolumeMaxExt(const UINT64 volume)=0;
   //--- category
   virtual LPCWSTR   Category(void) const=0;
   virtual MTAPIRES  Category(LPCWSTR category)=0;
   //--- exchange
   virtual LPCWSTR   Exchange(void) const=0;
   virtual MTAPIRES  Exchange(LPCWSTR exchange)=0;
   //--- CFI
   virtual LPCWSTR   CFI(void) const=0;
   virtual MTAPIRES  CFI(LPCWSTR cfi)=0;
   //--- Sector
   virtual UINT      Sector(void) const=0;
   virtual MTAPIRES  Sector(const UINT sector)=0;
   //--- Industry
   virtual UINT      Industry(void) const=0;
   virtual MTAPIRES  Industry(const UINT industry)=0;
   //--- Country - ISO 3166-1 alpha-3 code
   virtual LPCWSTR   Country(void) const=0;
   virtual MTAPIRES  Country(LPCWSTR country)=0;
   //--- Delay for subscriptions
   virtual UINT      SubscriptionsDelay(void) const=0;
   virtual MTAPIRES  SubscriptionsDelay(const UINT delay)=0;
   //--- Days in year
   virtual UINT      SwapYearDays(void) const=0;
   virtual MTAPIRES  SwapYearDays(const UINT days)=0;
   //--- swap flags
   virtual UINT      SwapFlags(void) const=0;
   virtual MTAPIRES  SwapFlags(const UINT flags)=0;
   //--- swap rate for Sunday
   virtual double    SwapRateSunday(void) const=0;
   virtual MTAPIRES  SwapRateSunday(const double rate)=0;
   //--- swap rate for Monday
   virtual double    SwapRateMonday(void) const=0;
   virtual MTAPIRES  SwapRateMonday(const double rate)=0;
   //--- swap rate for Tuesday
   virtual double    SwapRateTuesday(void) const=0;
   virtual MTAPIRES  SwapRateTuesday(const double rate)=0;
   //--- swap rate for Wednesday
   virtual double    SwapRateWednesday(void) const=0;
   virtual MTAPIRES  SwapRateWednesday(const double rate)=0;
   //--- swap rate for Thursday
   virtual double    SwapRateThursday(void) const=0;
   virtual MTAPIRES  SwapRateThursday(const double rate)=0;
   //--- swap rate for Friday
   virtual double    SwapRateFriday(void) const=0;
   virtual MTAPIRES  SwapRateFriday(const double rate)=0;
   //--- swap rate for Saturday
   virtual double    SwapRateSaturday(void) const=0;
   virtual MTAPIRES  SwapRateSaturday(const double rate)=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTConSymbol(void) {}
  };

class IMTConSymbolSink
  {
public:
   virtual void      OnSymbolAdd(const IMTConSymbol*    /*config*/) {  }
   virtual void      OnSymbolUpdate(const IMTConSymbol* /*config*/) {  }
   virtual void      OnSymbolDelete(const IMTConSymbol* /*config*/) {  }
   virtual void      OnSymbolSync(void)                             {  }
   //--- config modification hooks (only for Server API)
   virtual MTAPIRES  HookSymbolAdd(const UINT64 /*login*/,IMTConSymbol* /*new_cfg*/)                                { return(MT_RET_OK); }
   virtual MTAPIRES  HookSymbolUpdate(const UINT64 /*login*/,const IMTConSymbol* /*cfg*/,IMTConSymbol* /*new_cfg*/) { return(MT_RET_OK); }
   virtual MTAPIRES  HookSymbolDelete(const UINT64 /*login*/,const IMTConSymbol* /*cfg*/)                           { return(MT_RET_OK); }
  };

class IMTConGroupSymbol
  {
public:
   //--- Requests Execution flags
   enum EnREFlags
     {
      RE_FLAGS_NONE           =0,  // none
      RE_FLAGS_ORDER          =1,  // confirm orders after price confirmation
      //--- enumeration borders
      RE_FLAGS_ALL            =RE_FLAGS_ORDER
     };
   //--- Permissions
   enum EnPermissionsFlags
     {
      PERMISSION_NONE         =0,  // none
      PERMISSION_BOOK         =1,  // allow books for group
      //--- enumeration borders
      PERMISSION_DEFAULT      =PERMISSION_BOOK,
      PERMISSION_ALL          =PERMISSION_BOOK
     };

public:
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTConGroupSymbol* group)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- setup default values for all fields
   virtual MTAPIRES  Default(void)=0;
   //--- symbol or symbol groups path
   virtual LPCWSTR   Path(void) const=0;
   virtual MTAPIRES  Path(LPCWSTR path)=0;
   //--- IMTConSymbol::EnTradeMode
   virtual UINT      TradeMode(void) const=0;
   virtual MTAPIRES  TradeMode(const UINT mode)=0;
   virtual UINT      TradeModeDefault(void) const=0;
   //--- IMTConSymbol::EnCalcMode
   virtual UINT      ExecMode(void) const=0;
   virtual MTAPIRES  ExecMode(const UINT mode)=0;
   virtual UINT      ExecModeDefault(void) const=0;
   //--- IMTConSymbol::EnTradeFillingFlags
   virtual UINT      FillFlags(void) const=0;
   virtual MTAPIRES  FillFlags(const UINT flags)=0;
   virtual UINT      FillFlagsDefault(void) const=0;
   //--- IMTConSymbol::EnTradeExpirationFlags
   virtual UINT      ExpirFlags(void) const=0;
   virtual MTAPIRES  ExpirFlags(const UINT flags)=0;
   virtual UINT      ExpirFlagsDefault(void) const=0;
   //--- spread difference (0 - floating spread)
   virtual INT       SpreadDiff(void) const=0;
   virtual MTAPIRES  SpreadDiff(const INT spread)=0;
   virtual INT       SpreadDiffDefault(void) const=0;
   //--- spread difference balance
   virtual INT       SpreadDiffBalance(void) const=0;
   virtual MTAPIRES  SpreadDiffBalance(const INT spread)=0;
   virtual INT       SpreadDiffBalanceDefault(void) const=0;
   //--- stops level
   virtual INT       StopsLevel(void) const=0;
   virtual MTAPIRES  StopsLevel(const INT level)=0;
   virtual INT       StopsLevelDefault(void) const=0;
   //--- freeze level
   virtual INT       FreezeLevel(void) const=0;
   virtual MTAPIRES  FreezeLevel(const INT level)=0;
   virtual INT       FreezeLevelDefault(void) const=0;
   //--- minimal volume
   virtual UINT64    VolumeMin(void) const=0;
   virtual MTAPIRES  VolumeMin(const UINT64 volume)=0;
   virtual UINT64    VolumeMinDefault(void) const=0;
   //--- maximal volume
   virtual UINT64    VolumeMax(void) const=0;
   virtual MTAPIRES  VolumeMax(const UINT64 volume)=0;
   virtual UINT64    VolumeMaxDefault(void) const=0;
   //--- volume step
   virtual UINT64    VolumeStep(void) const=0;
   virtual MTAPIRES  VolumeStep(const UINT64 volume)=0;
   virtual UINT64    VolumeStepDefault(void) const=0;
   //--- cumulative positions and orders limit
   virtual UINT64    VolumeLimit(void) const=0;
   virtual MTAPIRES  VolumeLimit(const UINT64 volume)=0;
   virtual UINT64    VolumeLimitDefault(void) const=0;
   //--- IMTConSymbol::EnMarginFlags
   virtual UINT      MarginFlags(void) const=0;
   virtual MTAPIRES  MarginFlags(const UINT flags)=0;
   virtual UINT      MarginFlagsDefault(void) const=0;
   //--- initial margin
   virtual double    MarginInitial(void) const=0;
   virtual MTAPIRES  MarginInitial(const double margin)=0;
   virtual double    MarginInitialDefault(void) const=0;
   //--- maintenance margin
   virtual double    MarginMaintenance(void) const=0;
   virtual MTAPIRES  MarginMaintenance(const double margin)=0;
   virtual double    MarginMaintenanceDefault(void) const=0;
   //--- long orders and positions margin rate
   virtual double    MarginLong(void) const=0;
   virtual MTAPIRES  MarginLong(const double margin)=0;
   virtual double    MarginLongDefault(void) const=0;
   //--- short orders and positions margin rate
   virtual double    MarginShort(void) const=0;
   virtual MTAPIRES  MarginShort(const double margin)=0;
   virtual double    MarginShortDefault(void) const=0;
   //--- limit orders and positions margin rate
   virtual double    MarginLimit(void) const=0;
   virtual MTAPIRES  MarginLimit(const double margin)=0;
   virtual double    MarginLimitDefault(void) const=0;
   //--- stop orders and positions margin rate
   virtual double    MarginStop(void) const=0;
   virtual MTAPIRES  MarginStop(const double margin)=0;
   virtual double    MarginStopDefault(void) const=0;
   //--- stop-limit orders and positions margin rate
   virtual double    MarginStopLimit(void) const=0;
   virtual MTAPIRES  MarginStopLimit(const double margin)=0;
   virtual double    MarginStopLimitDefault(void) const=0;
   //--- IMTConSymbol::EnSwapMode
   virtual UINT      SwapMode(void) const=0;
   virtual MTAPIRES  SwapMode(const UINT mode)=0;
   virtual UINT      SwapModeDefault(void) const=0;
   //--- long positions swaps rate
   virtual double    SwapLong(void) const=0;
   virtual MTAPIRES  SwapLong(const double swap)=0;
   virtual double    SwapLongDefault(void) const=0;
   //--- short positions swaps rate
   virtual double    SwapShort(void) const=0;
   virtual MTAPIRES  SwapShort(const double swap)=0;
   virtual double    SwapShortDefault(void) const=0;
   //--- 3 time swaps day, EnSwapDay (obsolete)
   virtual int       Swap3Day(void) const=0;
   virtual MTAPIRES  Swap3Day(const int day)=0;
   virtual int       Swap3DayDefault(void) const=0;
   //--- request execution timeout
   virtual UINT      RETimeout(void) const=0;
   virtual MTAPIRES  RETimeout(const UINT timeout)=0;
   virtual UINT      RETimeoutDefault(void) const=0;
   //--- instant execution check mode
   virtual UINT      IECheckMode(void) const=0;
   virtual MTAPIRES  IECheckMode(const UINT mode)=0;
   virtual UINT      IECheckModeDefault(void) const=0;
   //--- instant execution timeout
   virtual UINT      IETimeout(void) const=0;
   virtual MTAPIRES  IETimeout(const UINT timeout)=0;
   virtual UINT      IETimeoutDefault(void) const=0;
   //--- instant execution profit slippage
   virtual UINT      IESlipProfit(void) const=0;
   virtual MTAPIRES  IESlipProfit(const UINT slippage)=0;
   virtual UINT      IESlipProfitDefault(void) const=0;
   //--- instant execution losing slippage
   virtual UINT      IESlipLosing(void) const=0;
   virtual MTAPIRES  IESlipLosing(const UINT slippage)=0;
   virtual UINT      IESlipLosingDefault(void) const=0;
   //--- instant execution max volume
   virtual UINT64    IEVolumeMax(void) const=0;
   virtual MTAPIRES  IEVolumeMax(const UINT64 volume)=0;
   virtual UINT64    IEVolumeMaxDefault(void) const=0;
   //--- IMTConSymbol::EnOrderFlags
   virtual UINT      OrderFlags(void) const=0;
   virtual MTAPIRES  OrderFlags(const UINT flags)=0;
   virtual UINT      OrderFlagsDefault(void) const=0;
   //--- orders and positions margin rates
   virtual double    MarginRateInitial(const UINT type) const=0;
   virtual MTAPIRES  MarginRateInitial(const UINT type,const double margin_rate)=0;
   virtual double    MarginRateInitialDefault(void) const=0;
   //--- orders and positions margin rates
   virtual double    MarginRateMaintenance(const UINT type) const=0;
   virtual MTAPIRES  MarginRateMaintenance(const UINT type,const double margin_rate)=0;
   virtual double    MarginRateMaintenanceDefault(void) const=0;
   //--- trade symbol liquidity rate in margin calculation
   virtual double    MarginRateLiquidity(void) const=0;
   virtual MTAPIRES  MarginRateLiquidity(const double rate)=0;
   virtual double    MarginRateLiquidityDefault(void) const=0;
   //--- request execution flags IMTConGroupSymbol::EnREFlags
   virtual UINT      REFlags(void) const=0;
   virtual MTAPIRES  REFlags(const UINT flags)=0;
   virtual UINT      REFlagsDefault(void) const=0;
   //--- hedged positions margin rate
   virtual double    MarginHedged(void) const=0;
   virtual MTAPIRES  MarginHedged(const double margin)=0;
   virtual double    MarginHedgedDefault(void) const=0;
   //--- permissions flags
   virtual UINT      PermissionsFlags(void) const=0;
   virtual MTAPIRES  PermissionsFlags(const UINT flags)=0;
   //--- margin currency rate
   virtual double    MarginRateCurrency(void) const=0;
   virtual MTAPIRES  MarginRateCurrency(const double margin_rate)=0;
   virtual double    MarginRateCurrencyDefault(void) const=0;
   //--- book depth limit
   virtual UINT      BookDepthLimit(void) const=0;
   virtual MTAPIRES  BookDepthLimit(const UINT depth)=0;
   //--- instant execution flags
   virtual UINT      IEFlags(void) const=0;
   virtual MTAPIRES  IEFlags(const UINT flags)=0;
   virtual UINT      IEFlagsDefault(void) const=0;
   //--- minimal volume
   virtual UINT64    VolumeMinExt(void) const=0;
   virtual MTAPIRES  VolumeMinExt(const UINT64 volume)=0;
   virtual UINT64    VolumeMinExtDefault(void) const=0;
   //--- maximal volume
   virtual UINT64    VolumeMaxExt(void) const=0;
   virtual MTAPIRES  VolumeMaxExt(const UINT64 volume)=0;
   virtual UINT64    VolumeMaxExtDefault(void) const=0;
   //--- volume step
   virtual UINT64    VolumeStepExt(void) const=0;
   virtual MTAPIRES  VolumeStepExt(const UINT64 volume)=0;
   virtual UINT64    VolumeStepExtDefault(void) const=0;
   //--- cumulative positions and orders limit
   virtual UINT64    VolumeLimitExt(void) const=0;
   virtual MTAPIRES  VolumeLimitExt(const UINT64 volume)=0;
   virtual UINT64    VolumeLimitExtDefault(void) const=0;
   //--- instant execution max volume
   virtual UINT64    IEVolumeMaxExt(void) const=0;
   virtual MTAPIRES  IEVolumeMaxExt(const UINT64 volume)=0;
   virtual UINT64    IEVolumeMaxExtDefault(void) const=0;
   //--- days in year
   virtual UINT      SwapYearDays(void) const=0;
   virtual MTAPIRES  SwapYearDays(const UINT days)=0;
   virtual UINT      SwapYearDaysDefault(void) const=0;
   //--- swap flags
   virtual UINT      SwapFlags(void) const=0;
   virtual MTAPIRES  SwapFlags(const UINT flags)=0;
   virtual UINT      SwapFlagsDefault(void) const=0;
   //--- swap rate for Sunday
   virtual double    SwapRateSunday(void) const=0;
   virtual MTAPIRES  SwapRateSunday(const double rate)=0;
   virtual double    SwapRateSundayDefault(void) const=0;
   //--- swap rate for Monday
   virtual double    SwapRateMonday(void) const=0;
   virtual MTAPIRES  SwapRateMonday(const double rate)=0;
   virtual double    SwapRateMondayDefault(void) const=0;
   //--- swap rate for Tuesday
   virtual double    SwapRateTuesday(void) const=0;
   virtual MTAPIRES  SwapRateTuesday(const double rate)=0;
   virtual double    SwapRateTuesdayDefault(void) const=0;
   //--- swap rate for Wednesday
   virtual double    SwapRateWednesday(void) const=0;
   virtual MTAPIRES  SwapRateWednesday(const double rate)=0;
   virtual double    SwapRateWednesdayDefault(void) const=0;
   //--- swap rate for Thursday
   virtual double    SwapRateThursday(void) const=0;
   virtual MTAPIRES  SwapRateThursday(const double rate)=0;
   virtual double    SwapRateThursdayDefault(void) const=0;
   //--- swap rate for Friday
   virtual double    SwapRateFriday(void) const=0;
   virtual MTAPIRES  SwapRateFriday(const double rate)=0;
   virtual double    SwapRateFridayDefault(void) const=0;
   //--- swap rate for Saturday
   virtual double    SwapRateSaturday(void) const=0;
   virtual MTAPIRES  SwapRateSaturday(const double rate)=0;
   virtual double    SwapRateSaturdayDefault(void) const=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTConGroupSymbol(void) {}
  };

class IMTConCommTier
  {
public:
   //--- commission charge mode
   enum EnCommissionMode
     {
      COMM_MONEY_DEPOSIT       =0,  // in money, in group deposit currency
      COMM_MONEY_SYMBOL_BASE   =1,  // in money, in symbol base currency
      COMM_MONEY_SYMBOL_PROFIT =2,  // in money, in symbol profit currency
      COMM_MONEY_SYMBOL_MARGIN =3,  // in money, in symbol margin currency
      COMM_PIPS                =4,  // in pips
      COMM_PERCENT             =5,  // in percent
      COMM_MONEY_SPECIFIED     =6,  // in money, in specified currency
      COMM_PERCENT_PROFIT      =7,  // in profit percent
      //--- enumeration borders
      COMM_FIRST               =COMM_MONEY_DEPOSIT,
      COMM_LAST                =COMM_PERCENT_PROFIT
     };
   //--- commission type by volume
   enum EnCommissionVolumeType
     {
      COMM_TYPE_DEAL           =0,  // commission per deal
      COMM_TYPE_VOLUME         =1,  // commission per volume
      //--- enumeration borders
      COMM_TYPE_FIRST          =COMM_TYPE_DEAL,
      COMM_TYPE_LAST           =COMM_TYPE_VOLUME
     };
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTConCommTier* group)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- EnCommissionMode
   virtual UINT      Mode(void) const=0;
   virtual MTAPIRES  Mode(const UINT mode)=0;
   //--- EnCommissionVolumeType
   virtual UINT      Type(void) const=0;
   virtual MTAPIRES  Type(const UINT type)=0;
   //--- commission value
   virtual double    Value(void) const=0;
   virtual MTAPIRES  Value(const double value)=0;
   //--- minimal commission value
   virtual double    Minimal(void) const=0;
   virtual MTAPIRES  Minimal(const double value)=0;
   //--- tier range from
   virtual double    RangeFrom(void) const=0;
   virtual MTAPIRES  RangeFrom(const double value)=0;
   //--- tier range to
   virtual double    RangeTo(void) const=0;
   virtual MTAPIRES  RangeTo(const double value)=0;
   //--- commission currency (for Mode==COMM_MONEY_SPECIFIED)
   virtual LPCWSTR   Currency(void) const=0;
   virtual MTAPIRES  Currency(LPCWSTR currency)=0;
   //--- maximal commission value
   virtual double    Maximal(void) const=0;
   virtual MTAPIRES  Maximal(const double value)=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTConCommTier(void) {}
  };

class IMTConCommission
  {
public:
   //--- commission mode
   enum EnCommMode
     {
      COMM_STANDARD            =0,  // standard commission
      COMM_AGENT               =1,  // agent commission
      COMM_FEE                 =2,  // fee
      //--- enumeration borders
      COMM_FIRST               =COMM_STANDARD,
      COMM_LAST                =COMM_FEE
     };
   //--- commission range mode
   enum EnCommRangeMode
     {
      COMM_RANGE_VOLUME         =0,  // range in volumes
      COMM_RANGE_TURNOVER_MONEY =1,  // turnover in money
      COMM_RANGE_TURNOVER_VOLUME=2,  // turnover in volume
      COMM_RANGE_VALUE          =3,  // range in values
      COMM_RANGE_PROFIT         =4,  // range in profit
      //--- enumeration borders
      COMM_RANGE_FIRST         =COMM_RANGE_VOLUME,
      COMM_RANGE_LAST          =COMM_RANGE_PROFIT
     };
   //--- commission charge modes
   enum EnCommChargeMode
     {
      COMM_CHARGE_DAILY        =0, // charge at the end of daily
      COMM_CHARGE_MONTHLY      =1, // charge at the end of month
      COMM_CHARGE_INSTANT      =2, // charge instantly
      //--- enumeration borders
      COMM_CHARGE_FIRST        =COMM_CHARGE_DAILY,
      COMM_CHARGE_LAST         =COMM_CHARGE_INSTANT
     };
   //--- deal entry mode
   enum EnCommEntryMode
     {
      COMM_ENTRY_ALL           =0, // both in and out
      COMM_ENTRY_IN            =1, // in deals
      COMM_ENTRY_OUT           =2, // out deals
      //--- enumeration borders
      COMM_ENTRY_FIRST         =COMM_ENTRY_ALL,
      COMM_ENTRY_LAST          =COMM_ENTRY_OUT
     };
   //--- deal action mode
   enum EnCommActionMode
     {
      COMM_ACTION_ALL          =0,
      COMM_ACTION_BUY          =1,
      COMM_ACTION_SELL         =2,
      //--- enumeration borders
      COMM_ACTION_FIRST        =COMM_ACTION_ALL,
      COMM_ACTION_LAST         =COMM_ACTION_SELL
     };
   //--- deal profit mode
   enum EnCommProfitMode
     {
      COMM_PROFIT_ALL          =0,
      COMM_PROFIT_PROFIT       =1,
      COMM_PROFIT_LOSS         =2,
      //--- enumeration borders
      COMM_PROFIT_FIRST        =COMM_PROFIT_ALL,
      COMM_PROFIT_LAST         =COMM_PROFIT_LOSS
     };
   //--- deal reason
   enum EnCommReasonFlags
     {
      COMM_REASON_FLAG_NONE            =0x00000000,   // none
      COMM_REASON_FLAG_CLIENT          =0x00000001,   // client terminal
      COMM_REASON_FLAG_EXPERT          =0x00000002,   // expert
      COMM_REASON_FLAG_DEALER          =0x00000004,   // dealer
      COMM_REASON_FLAG_EXTERNAL_CLIENT =0x00000008,   // external client
      COMM_REASON_FLAG_MOBILE          =0x00000010,   // mobile terminal
      COMM_REASON_FLAG_WEB             =0x00000020,   // web terminal
      COMM_REASON_FLAG_SIGNAL          =0x00000040,   // signal service
      //--- enumeration borders
      COMM_REASON_FLAG_ALL             =COMM_REASON_FLAG_CLIENT | COMM_REASON_FLAG_EXPERT | COMM_REASON_FLAG_DEALER | COMM_REASON_FLAG_EXTERNAL_CLIENT | COMM_REASON_FLAG_MOBILE | COMM_REASON_FLAG_WEB | COMM_REASON_FLAG_SIGNAL
     };
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTConCommission* group)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- commission name
   virtual LPCWSTR   Name(void) const=0;
   virtual MTAPIRES  Name(LPCWSTR name)=0;
   //--- description
   virtual LPCWSTR   Description(void) const=0;
   virtual MTAPIRES  Description(LPCWSTR descr)=0;
   //--- symbols path
   virtual LPCWSTR   Path(void) const=0;
   virtual MTAPIRES  Path(LPCWSTR path)=0;
   //--- EnCommMode
   virtual UINT      Mode(void) const=0;
   virtual MTAPIRES  Mode(const UINT mode)=0;
   //--- EnCommRangeMode
   virtual UINT      RangeMode(void) const=0;
   virtual MTAPIRES  RangeMode(const UINT mode)=0;
   //--- EnCommChargeMode
   virtual UINT      ChargeMode(void) const=0;
   virtual MTAPIRES  ChargeMode(const UINT mode)=0;
   //--- commission tiers
   virtual MTAPIRES  TierAdd(IMTConCommTier* tier)=0;
   virtual MTAPIRES  TierUpdate(const UINT pos,const IMTConCommTier* tier)=0;
   virtual MTAPIRES  TierDelete(const UINT pos)=0;
   virtual MTAPIRES  TierClear(void)=0;
   virtual MTAPIRES  TierShift(const UINT pos,const int shift)=0;
   virtual UINT      TierTotal(void) const=0;
   virtual MTAPIRES  TierNext(const UINT pos,IMTConCommTier* tier) const=0;
   //---- turnover calculation currency
   virtual LPCWSTR   TurnoverCurrency(void) const=0;
   virtual MTAPIRES  TurnoverCurrency(LPCWSTR currency)=0;
   //--- EnCommEntryMode
   virtual UINT      EntryMode(void) const=0;
   virtual MTAPIRES  EntryMode(const UINT mode)=0;
   //--- EnCommActionMode
   virtual UINT      ActionMode(void) const=0;
   virtual MTAPIRES  ActionMode(const UINT mode)=0;
   //--- EnCommProfitMode
   virtual UINT      ProfitMode(void) const=0;
   virtual MTAPIRES  ProfitMode(const UINT mode)=0;
   //--- EnCommReasonFlags
   virtual UINT      ReasonFlags(void) const=0;
   virtual MTAPIRES  ReasonFlags(const UINT flags)=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTConCommission(void) {}
  };

class IMTConGroup
  {
public:
   //--- group permissions flags
   enum EnPermissionsFlags
     {
      PERMISSION_NONE              =0x00000000,  // default
      PERMISSION_CERT_CONFIRM      =0x00000001,  // certificate confirmation neccessary
      PERMISSION_ENABLE_CONNECTION =0x00000002,  // clients connections allowed
      PERMISSION_RESET_PASSWORD    =0x00000004,  // reset password after first logon
      PERMISSION_FORCED_OTP_USAGE  =0x00000008,  // forced usage OTP
      PERMISSION_RISK_WARNING      =0x00000010,  // show risk warning window on start
      PERMISSION_REGULATION_PROTECT=0x00000020,  // country-specific regulatory protection
      PERMISSION_NOTIFY_DEALS      =0x00000040,  // allow deals notification by push message
      PERMISSION_NOTIFY_ORDERS     =0x00000080,  // allow orders notification by push message
      PERMISSION_NOTIFY_BALANCES   =0x00000100,  // allow balances notification by push message
      //--- all push notifications
      PERMISSION_NOTIFY_ALL        =PERMISSION_NOTIFY_DEALS|PERMISSION_NOTIFY_ORDERS|PERMISSION_NOTIFY_BALANCES,
      //--- enumeration borders
      PERMISSION_ALL         =PERMISSION_CERT_CONFIRM     | PERMISSION_ENABLE_CONNECTION | PERMISSION_RESET_PASSWORD|
      PERMISSION_FORCED_OTP_USAGE | PERMISSION_RISK_WARNING      | PERMISSION_REGULATION_PROTECT | PERMISSION_NOTIFY_DEALS |
      PERMISSION_NOTIFY_ORDERS | PERMISSION_NOTIFY_BALANCES
     };
   //--- authorization mode
   enum EnAuthMode
     {
      AUTH_STANDARD               =0,  // standard authorization
      AUTH_RSA1024                =1,  // RSA1024 certificate
      AUTH_RSA2048                =2,  // RSA2048 certificate
      AUTH_RSA_CUSTOM             =3,  // RSA custom
      //--- enumeration borders
      AUTH_FIRST                  =AUTH_STANDARD,
      AUTH_LAST                   =AUTH_RSA2048
     };
   //--- One-Time-Password mode
   enum EnAuthOTPMode
     {
      AUTH_OTP_DISABLED           =0,
      AUTH_OTP_TOTP_SHA256        =1,
      AUTH_OTP_TOTP_SHA256_WEB    =2,
      //--- enumeration borders
      AUTH_OTP_FIRST              =AUTH_OTP_DISABLED,
      AUTH_OTP_LAST               =AUTH_OTP_TOTP_SHA256_WEB
     };
   //--- reports generation mode
   enum EnReportsMode
     {
      REPORTS_DISABLED            =0,  // reports disabled
      REPORTS_STANDARD            =1,  // standard mode
      //--- enumeration borders
      REPORTS_FIRST               =REPORTS_DISABLED,
      REPORTS_LAST                =REPORTS_STANDARD
     };
   //--- reports generation flags
   enum EnReportsFlags
     {
      REPORTSFLAGS_NONE           =0,  // none
      REPORTSFLAGS_EMAIL          =1,  // send reports through email
      REPORTSFLAGS_SUPPORT        =2,  // send reports copies on support email
      REPORTSFLAGS_STATEMENTS     =4,  // generate reports
      //--- enumeration borders
      REPORTSFLAGS_ALL            =REPORTSFLAGS_NONE|REPORTSFLAGS_EMAIL|REPORTSFLAGS_STATEMENTS
     };
   //--- news modes
   enum EnNewsMode
     {
      NEWS_MODE_DISABLED          =0,  // disable news
      NEWS_MODE_HEADERS           =1,  // enable only news headers
      NEWS_MODE_FULL              =2,  // enable full news
      //--- enumeration borders
      NEWS_MODE_FIRST             =NEWS_MODE_DISABLED,
      NEWS_MODE_LAST              =NEWS_MODE_FULL
     };
   //--- internal email modes
   enum EnMailMode
     {
      MAIL_MODE_DISABLED          =0,  // disable internal email
      MAIL_MODE_FULL              =1,  // enable internal email
      //--- enumeration borders
      MAIL_MODE_FIRST             =MAIL_MODE_DISABLED,
      MAIL_MODE_LAST              =MAIL_MODE_FULL
     };
   //--- client history limits
   enum EnHistoryLimit
     {
      TRADE_HISTORY_ALL           =0,  // unlimited
      TRADE_HISTORY_MONTHS_1      =1,  // one month
      TRADE_HISTORY_MONTHS_3      =2,  // 3 months
      TRADE_HISTORY_MONTHS_6      =3,  // 6 months
      TRADE_HISTORY_YEAR_1        =4,  // 1 year
      TRADE_HISTORY_YEAR_2        =5,  // 2 years
      TRADE_HISTORY_YEAR_3        =6,  // 3 years
      //--- enumeration borders
      TRADE_HISTORY_FIRST         =TRADE_HISTORY_ALL,
      TRADE_HISTORY_LAST          =TRADE_HISTORY_YEAR_3
     };
   //--- free margin calculation modes
   enum EnFreeMarginMode
     {
      FREE_MARGIN_NOT_USE_PL      =0,  // don't use floating profit and loss
      FREE_MARGIN_USE_PL          =1,  // use floating profit and loss
      FREE_MARGIN_PROFIT          =2,  // use floating profit only
      FREE_MARGIN_LOSS            =3,  // use floating loss only
      //--- enumeration borders
      FREE_MARGIN_FIRST           =FREE_MARGIN_NOT_USE_PL,
      FREE_MARGIN_LAST            =FREE_MARGIN_LOSS
     };
   //--- EnTransferMode
   enum EnTransferMode
     {
      TRANSFER_MODE_DISABLED      =0,
      TRANSFER_MODE_NAME          =1,
      TRANSFER_MODE_GROUP         =2,
      TRANSFER_MODE_NAME_GROUP    =3,
      //--- enumeration borders
      TRANSFER_MODE_FIRST         =TRANSFER_MODE_DISABLED,
      TRANSFER_MODE_LAST          =TRANSFER_MODE_NAME_GROUP
     };
   //--- stop-out mode
   enum EnStopOutMode
     {
      STOPOUT_PERCENT             =0,  // stop-out in percent
      STOPOUT_MONEY               =1,  // stop-out in money
      //--- enumeration borders
      STOPOUT_FIRST               =STOPOUT_PERCENT,
      STOPOUT_LAST                =STOPOUT_MONEY
     };
   //--- fixed profit calculation in free margin
   enum EnMarginFreeProfitMode
     {
      FREE_MARGIN_PROFIT_PL       =0,  // both fixed loss and profit on free margin
      FREE_MARGIN_PROFIT_LOSS     =1,  // only fixed loss on free margin
      //--- enumeration borders
      FREE_MARGIN_PROFIT_FIRST    =FREE_MARGIN_PROFIT_PL,
      FREE_MARGIN_PROFIT_LAST     =FREE_MARGIN_PROFIT_LOSS
     };
   //--- group risk management mode 
   enum EnMarginMode
     {
      MARGIN_MODE_RETAIL           =0, // Retail FX, Retail CFD, Retail Futures
      MARGIN_MODE_EXCHANGE_DISCOUNT=1, // Exchange, margin discount rates based 
      MARGIN_MODE_RETAIL_HEDGED    =2, // Retail FX, Retail CFD, Retail Futures with hedged positions
      //--- enumeration borders
      MARGIN_MODE_FIRST           =MARGIN_MODE_RETAIL,
      MARGIN_MODE_LAST            =MARGIN_MODE_RETAIL_HEDGED
     };
   //--- margin calculation flags
   enum EnMarginFlags
     {
      MARGIN_FLAGS_NONE           =0,        // none
      MARGIN_FLAGS_CLEAR_ACC      =1,        // clear accumulated profit at end of day
      //--- enumeration borders
      MARGIN_FLAGS_ALL            =MARGIN_FLAGS_CLEAR_ACC
     };
   //--- trade rights flags
   enum EnTradeFlags
     {
      TRADEFLAGS_NONE             =0x00000000,  // none
      TRADEFLAGS_SWAPS            =0x00000001,  // allow swaps charges
      TRADEFLAGS_TRAILING         =0x00000002,  // allow trailing stops
      TRADEFLAGS_EXPERTS          =0x00000004,  // allow expert advisors
      TRADEFLAGS_EXPIRATION       =0x00000008,  // allow orders expiration
      TRADEFLAGS_SIGNALS_ALL      =0x00000010,  // allow trade signals
      TRADEFLAGS_SIGNALS_OWN      =0x00000020,  // allow trade signals only from own server
      TRADEFLAGS_SO_COMPENSATION  =0x00000040,  // allow negative balance compensation after stop out
      TRADEFLAGS_SO_FULLY_HEDGED  =0x00000080,  // allow stop out fully hegded accounts
      TRADEFLAGS_FIFO_CLOSE       =0x00000100,  // allow to close positions by FIFO rule
      TRADEFLAGS_HEDGE_PROHIBIT   =0x00000200,  // prohibit hedged positions
      TRADEFLAGS_DEAL_COST        =0x00000400,  // calculate and show deals cost
      TRADEFLAGS_SO_COMPENSATION_CREDIT=0x00000800, // allow credit compensation after stop out
      //--- enumeration borders
      TRADEFLAGS_DEFAULT          =TRADEFLAGS_SWAPS|TRADEFLAGS_TRAILING|TRADEFLAGS_EXPERTS|TRADEFLAGS_EXPIRATION|TRADEFLAGS_SIGNALS_ALL,
      TRADEFLAGS_ALL              =TRADEFLAGS_SWAPS|TRADEFLAGS_TRAILING|TRADEFLAGS_EXPERTS|TRADEFLAGS_EXPIRATION|TRADEFLAGS_SIGNALS_ALL|TRADEFLAGS_SIGNALS_OWN|
      TRADEFLAGS_SO_COMPENSATION|TRADEFLAGS_SO_FULLY_HEDGED|TRADEFLAGS_FIFO_CLOSE|TRADEFLAGS_HEDGE_PROHIBIT|TRADEFLAGS_DEAL_COST|TRADEFLAGS_SO_COMPENSATION_CREDIT
     };
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTConGroup* group)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- group name
   virtual LPCWSTR   Group(void) const=0;
   virtual MTAPIRES  Group(LPCWSTR group)=0;
   //--- group trade server ID
   virtual UINT64    Server(void) const=0;
   virtual MTAPIRES  Server(const UINT64 server)=0;
   //--- EnPermissionsFlags
   virtual UINT64    PermissionsFlags(void) const=0;
   virtual MTAPIRES  PermissionsFlags(const UINT64 flags)=0;
   //--- EnAuthMode
   virtual UINT      AuthMode(void) const=0;
   virtual MTAPIRES  AuthMode(const UINT mode)=0;
   //--- minimal password length
   virtual UINT      AuthPasswordMin(void) const=0;
   virtual MTAPIRES  AuthPasswordMin(const UINT mode)=0;
   //--- company name
   virtual LPCWSTR   Company(void) const=0;
   virtual MTAPIRES  Company(LPCWSTR company)=0;
   //--- company web page URL
   virtual LPCWSTR   CompanyPage(void) const=0;
   virtual MTAPIRES  CompanyPage(LPCWSTR page)=0;
   //--- company email
   virtual LPCWSTR   CompanyEmail(void) const=0;
   virtual MTAPIRES  CompanyEmail(LPCWSTR email)=0;
   //--- company support site URL
   virtual LPCWSTR   CompanySupportPage(void) const=0;
   virtual MTAPIRES  CompanySupportPage(LPCWSTR page)=0;
   //--- company support email
   virtual LPCWSTR   CompanySupportEmail(void) const=0;
   virtual MTAPIRES  CompanySupportEmail(LPCWSTR email)=0;
   //--- company catalog name (for reports and email templates)
   virtual LPCWSTR   CompanyCatalog(void) const=0;
   virtual MTAPIRES  CompanyCatalog(LPCWSTR catalog)=0;
   //--- deposit currency
   virtual LPCWSTR   Currency(void) const=0;
   virtual MTAPIRES  Currency(LPCWSTR currency)=0;
   virtual UINT      CurrencyDigits(void) const=0;
   //--- EnReportsMode
   virtual UINT      ReportsMode(void) const=0;
   virtual MTAPIRES  ReportsMode(const UINT mode)=0;
   //--- EnReportsFlags
   virtual UINT64    ReportsFlags(void) const=0;
   virtual MTAPIRES  ReportsFlags(const UINT64 flags)=0;
   //--- reports SMTP server address:ports 
   virtual LPCWSTR   ReportsSMTP(void) const=0;
   virtual MTAPIRES  ReportsSMTP(LPCWSTR server)=0;
   //--- reports SMTP server login
   virtual LPCWSTR   ReportsSMTPLogin(void) const=0;
   virtual MTAPIRES  ReportsSMTPLogin(LPCWSTR login)=0;
   //--- reports SMTP server password
   virtual LPCWSTR   ReportsSMTPPass(void) const=0;
   virtual MTAPIRES  ReportsSMTPPass(LPCWSTR password)=0;
   //--- EnNewsMode
   virtual UINT      NewsMode(void) const=0;
   virtual MTAPIRES  NewsMode(const UINT mode)=0;
   //--- news category filter string
   virtual LPCWSTR   NewsCategory(void) const=0;
   virtual MTAPIRES  NewsCategory(LPCWSTR category)=0;
   //--- allowed news languages (Windows API LANGID used)
   virtual MTAPIRES  NewsLangAdd(const UINT language)=0;
   virtual MTAPIRES  NewsLangUpdate(const UINT pos,const UINT language)=0;
   virtual MTAPIRES  NewsLangDelete(const UINT pos)=0;
   virtual MTAPIRES  NewsLangClear(void)=0;
   virtual UINT      NewsLangTotal(void) const=0;
   virtual UINT      NewsLangNext(const UINT pos) const=0;
   //--- EnMailMode
   virtual UINT      MailMode(void) const=0;
   virtual MTAPIRES  MailMode(const UINT mode)=0;
   //--- EnTradeFlags
   virtual UINT64    TradeFlags(void) const=0;
   virtual MTAPIRES  TradeFlags(const UINT64 flags)=0;
   //--- interest rate for free deposit money
   virtual double    TradeInterestrate(void) const=0;
   virtual MTAPIRES  TradeInterestrate(const double rate)=0;
   //--- virtual credit
   virtual double    TradeVirtualCredit(void) const=0;
   virtual MTAPIRES  TradeVirtualCredit(const double credit)=0;
   //--- EnFreeMarginMode
   virtual UINT      MarginFreeMode(void) const=0;
   virtual MTAPIRES  MarginFreeMode(const UINT freemode)=0;
   //--- EnStopOutMode
   virtual UINT      MarginSOMode(void) const=0;
   virtual MTAPIRES  MarginSOMode(const UINT level)=0;
   //--- Margin Call level value
   virtual double    MarginCall(void) const=0;
   virtual MTAPIRES  MarginCall(const double level)=0;
   //--- Sto-Out level value
   virtual double    MarginStopOut(void) const=0;
   virtual MTAPIRES  MarginStopOut(const double level)=0;
   //--- default demo accounts leverage
   virtual UINT      DemoLeverage(void) const=0;
   virtual MTAPIRES  DemoLeverage(const UINT leverage)=0;
   //--- default demo accounts deposit
   virtual double    DemoDeposit(void) const=0;
   virtual MTAPIRES  DemoDeposit(const double deposit)=0;
   //--- EnHistoryLimit
   virtual UINT      LimitHistory(void) const=0;
   virtual MTAPIRES  LimitHistory(const UINT limit)=0;
   //--- max. order limit
   virtual UINT      LimitOrders(void) const=0;
   virtual MTAPIRES  LimitOrders(const UINT limit)=0;
   //--- max. selected symbols limit
   virtual UINT      LimitSymbols(void) const=0;
   virtual MTAPIRES  LimitSymbols(const UINT limit)=0;
   //--- commissions
   virtual MTAPIRES  CommissionAdd(IMTConCommission* commission)=0;
   virtual MTAPIRES  CommissionUpdate(const UINT pos,const IMTConCommission* commission)=0;
   virtual MTAPIRES  CommissionDelete(const UINT pos)=0;
   virtual MTAPIRES  CommissionClear(void)=0;
   virtual MTAPIRES  CommissionShift(const UINT pos,const int shift)=0;
   virtual UINT      CommissionTotal(void) const=0;
   virtual MTAPIRES  CommissionNext(const UINT pos,IMTConCommission* commission) const=0;
   virtual MTAPIRES  CommissionGet(LPCWSTR name,IMTConCommission* commission) const=0;
   //--- groups symbols settings
   virtual MTAPIRES  SymbolAdd(IMTConGroupSymbol* symbol)=0;
   virtual MTAPIRES  SymbolUpdate(const UINT pos,const IMTConGroupSymbol* symbol)=0;
   virtual MTAPIRES  SymbolDelete(const UINT pos)=0;
   virtual MTAPIRES  SymbolClear(void)=0;
   virtual MTAPIRES  SymbolShift(const UINT pos,const int shift)=0;
   virtual UINT      SymbolTotal(void) const=0;
   virtual MTAPIRES  SymbolNext(const UINT pos,IMTConGroupSymbol* symbol) const=0;
   virtual MTAPIRES  SymbolGet(LPCWSTR name,IMTConGroupSymbol* symbol) const=0;
   //--- margin free profit accounting mode
   virtual UINT      MarginFreeProfitMode(void) const=0;
   virtual MTAPIRES  MarginFreeProfitMode(const UINT mode)=0;
   //--- group risk management mode - EnMarginMode
   virtual UINT      MarginMode(void) const=0;
   virtual MTAPIRES  MarginMode(const UINT mode)=0;
   //--- OTP authentication mode - EnAuthOTPMode
   virtual UINT      AuthOTPMode(void) const=0;
   virtual MTAPIRES  AuthOTPMode(const UINT mode)=0;
   //--- deposit transfer mode - EnTransferMode
   virtual UINT      TradeTransferMode(void) const=0;
   virtual MTAPIRES  TradeTransferMode(const UINT mode)=0;
   //--- margin calculation flags EnMarginFlags
   virtual UINT64    MarginFlags(void) const=0;
   virtual MTAPIRES  MarginFlags(const UINT64 flags)=0;
   //--- max. positions limit
   virtual UINT      LimitPositions(void) const=0;
   virtual MTAPIRES  LimitPositions(const UINT limit)=0;
   //--- deposit currency digits
   virtual MTAPIRES  CurrencyDigitsSet(const UINT currency_digits)=0;
   //--- reports SMTP email account
   virtual LPCWSTR   ReportsEmail(void) const=0;
   virtual MTAPIRES  ReportsEmail(LPCWSTR email)=0;
   //--- company deposit URL
   virtual LPCWSTR   CompanyDepositPage(void) const=0;
   virtual MTAPIRES  CompanyDepositPage(LPCWSTR url)=0;
   //--- company deposit URL
   virtual LPCWSTR   CompanyWithdrawalPage(void) const=0;
   virtual MTAPIRES  CompanyWithdrawalPage(LPCWSTR url)=0;
   //--- demo groups in days, orders and positions will be deleted after this period
   virtual UINT      DemoInactivityPeriod(void) const=0;
   virtual MTAPIRES  DemoInactivityPeriod(const UINT period)=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTConGroup(void) {}
  };

class IMTConGroupSink
  {
public:
   virtual void      OnGroupAdd(const IMTConGroup*    /*config*/) {  }
   virtual void      OnGroupUpdate(const IMTConGroup* /*config*/) {  }
   virtual void      OnGroupDelete(const IMTConGroup* /*config*/) {  }
   virtual void      OnGroupSync(void)                            {  }
   //--- config modification hooks (only for Server API)
   virtual MTAPIRES  HookGroupAdd(const UINT64 /*login*/,IMTConGroup* /*new_cfg*/)                               { return(MT_RET_OK); }
   virtual MTAPIRES  HookGroupUpdate(const UINT64 /*login*/,const IMTConGroup* /*cfg*/,IMTConGroup* /*new_cfg*/) { return(MT_RET_OK); }
   virtual MTAPIRES  HookGroupDelete(const UINT64 /*login*/,const IMTConGroup* /*cfg*/)                          { return(MT_RET_OK); }
  };

class IMTConManagerAccess
  {
public:
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTConManagerAccess* access)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- ip address range from
   virtual LPCWSTR   From(void) const=0;
   virtual MTAPIRES  From(LPCWSTR name)=0;
   //--- ip address range to
   virtual LPCWSTR   To(void) const=0;
   virtual MTAPIRES  To(LPCWSTR value)=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTConManagerAccess(void) {}
  };

class IMTConManager
  {
public:
   enum EnManagerRights
     {
      RIGHT_ADMIN              =0,       // permission to connect using MetaTrader 5 Administrator
      RIGHT_MANAGER            =1,       // permission to connect using MetaTrader 5 Manager
      //---
      RIGHT_CFG_SERVERS        =10,      // permission to configure network
      RIGHT_CFG_ACCESS         =11,      // permission to configure IP access list
      RIGHT_CFG_TIME           =12,      // permission to configure operation time
      RIGHT_CFG_HOLIDAYS       =13,      // permission to configure holidays
      RIGHT_CFG_HST_SYNC       =14,      // permission to configure history charts synchronization
      RIGHT_CFG_SYMBOLS        =15,      // permission to configure symbols
      RIGHT_CFG_GROUPS         =16,      // permission to configure groups
      RIGHT_CFG_MANAGERS       =17,      // permission to configure managers' permissions
      RIGHT_CFG_DATAFEEDS      =18,      // permission to configure datafeeds
      RIGHT_CFG_REQUESTS       =19,      // permission to configure request routing
      RIGHT_SRV_JOURNALS       =20,      // permission to access server logs
      RIGHT_SRV_REPORTS        =21,      // permission to receive automatic server reports
      RIGHT_CHARTS             =22,      // permission to edit charts
      RIGHT_EMAIL              =23,      // permission to send emails
      RIGHT_ACCOUNTANT         =24,      // permission of accountant (charge/discharge)
      RIGHT_ACC_READ           =25,      // permission to access accounts
      RIGHT_ACC_DETAILS        =26,      // permission to access account personal details
      RIGHT_ACC_MANAGER        =27,      // permission to edit accounts
      RIGHT_ACC_ONLINE         =28,      // permission to view currently connected clients
      RIGHT_TRADES_READ        =29,      // permission to access orders and positions
      RIGHT_TRADES_MANAGER     =30,      // permission to edit trade records
      RIGHT_QUOTES             =31,      // permission to throw in quotes
      RIGHT_RISK_MANAGER       =32,      // permission of risk manager
      RIGHT_REPORTS            =33,      // permission to receive reports
      RIGHT_NEWS               =34,      // permission to send news
      RIGHT_CFG_GATEWAYS       =35,      // permission to configure gateways
      RIGHT_CFG_PLUGINS        =36,      // permission to configure plugins
      RIGHT_TRADES_DEALER      =37,      // permission of dealer
      RIGHT_CFG_REPORTS        =38,      // permission to configure reports
      RIGHT_EXPORT             =39,      // permission to export data
      RIGHT_SYMBOL_DETAILS     =40,      // permission to modify spread and execution mode
      RIGHT_TECHSUPPORT        =41,      // permission to access technical support page
      RIGHT_TRADES_SUPERVISOR  =42,      // permission of supervisor
      RIGHT_QUOTES_RAW         =43,      // permission to see raw quotes without spread difference
      RIGHT_MARKET             =44,      // permission to access applications market
      RIGHT_GRP_DETAILS_MARGIN =45,      // permission to edit groups margin
      RIGHT_NOTIFICATIONS      =46,      // permission to send notifications
      RIGHT_ACC_DELETE         =47,      // permission to delete accounts
      RIGHT_TRADES_DELETE      =48,      // permission to delete trades
      RIGHT_CONFIRM_ACTIONS    =49,      // permission to confirm actions by confirmation dialog
      RIGHT_CFG_ECN            =50,      // permission to configure ECN
      RIGHT_GRP_DETAILS_COMMISSION=51,   // permission to edit groups commission
      RIGHT_SUBSCRIPTIONS_VIEW =52,      // permission to view subscriptions
      RIGHT_SUBSCRIPTIONS_EDIT =53,      // permission to edit subscriptions
      RIGHT_CFG_FUNDS          =54,      // permission to configure funds
      RIGHT_CFG_MAILS          =55,      // permission to configure mail servers
      RIGHT_CFG_MESSENGERS     =56,      // permission to configure messengers
      RIGHT_CFG_KYC            =57,      // permission to configure KYC
      RIGHT_CFG_AUTOMATIONS    =58,      // permission to configure automation
      RIGHT_CFG_ALLOCATIONS    =59,      // permission to configure accounts allocation
      RIGHT_CFG_VPS            =60,      // permission to configure VPS
      RIGHT_CFG_PAYMENTS       =61,      // permission to configure payments
      RIGHT_ADMIN_COMPUTER     =62,      // permission to manage cluster computer
      RIGHT_CFG_WEB_SERVICES   =63,      // permission to configure web services
      RIGHT_FINTEZA_ACCESS     =64,      // permission to access Finteza
      RIGHT_FINTEZA_WEBSITES   =65,      // permission to access Finteza websites
      RIGHT_FINTEZA_CAMPAIGNS  =66,      // permission to access Finteza campaigns
      RIGHT_FINTEZA_REPORTS    =67,      // permission to access Finteza reports
      RIGHT_ACC_TECHNICAL      =70,      // permission to technical accounts
      RIGHT_ACC_TECHNICAL_MODIFY=71,     // permission to modify "technical account" option
      RIGHT_CLIENTS_ACCESS     =96,      // permission to access clients
      RIGHT_CLIENTS_CREATE     =97,      // permission to create clients
      RIGHT_CLIENTS_EDIT       =98,      // permission to edit clients
      RIGHT_CLIENTS_DELETE     =99,      // permission to delete clients
      RIGHT_DOCUMENTS_ACCESS   =100,     // permission to access documents
      RIGHT_DOCUMENTS_CREATE   =101,     // permission to create documents
      RIGHT_DOCUMENTS_EDIT     =102,     // permission to edits documents
      RIGHT_DOCUMENTS_DELETE   =103,     // permission to delete documents
      RIGHT_DOCUMENTS_FILES_ADD=104,     // permission to add files for documents
      RIGHT_DOCUMENTS_FILES_DELETE=105,  // permission to delete files for documents
      RIGHT_COMMENTS_ACCESS    =106,     // permission to access comments
      RIGHT_COMMENTS_CREATE    =107,     // permission to create comments
      RIGHT_COMMENTS_DELETE    =108,     // permission to delete comments
      RIGHT_CLIENTS_KYC        =109,     // permission to initate KYC check
      //--- enumeration borders
      RIGHT_FIRST              =RIGHT_ADMIN,
      RIGHT_LAST               =128,
     };
   //--- right flags
   enum EnManagerRightFlags
     {
      RIGHT_FLAGS_DENIED       =0,  // right denied
      RIGHT_FLAGS_GRANTED      =1,  // right granted
      //--- enumeration borders
      RIGHT_FLAGS_NONE         =0,
      RIGHT_FLAGS_ALL          =RIGHT_FLAGS_GRANTED
     };
   //--- data access limitation
   enum EnManagerLimit
     {
      MANAGER_LIMIT_ALL        =0,  // unlimited
      MANAGER_LIMIT_MONTHS_1   =1,  // 1 month
      MANAGER_LIMIT_MONTHS_3   =2,  // 3 months
      MANAGER_LIMIT_MONTHS_6   =3,  // 6 months
      MANAGER_LIMIT_YEAR_1     =4,  // 1 year
      MANAGER_LIMIT_YEAR_2     =5,  // 2 years
      MANAGER_LIMIT_YEAR_3     =6,  // 3 years
      //--- enumeration borders
      MANAGER_LIMIT_FIRST      =MANAGER_LIMIT_ALL,
      MANAGER_LIMIT_LAST       =MANAGER_LIMIT_YEAR_3
     };
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTConManager* manager)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- login
   virtual UINT64    Login(void) const=0;
   virtual MTAPIRES  Login(const UINT64 login)=0;
   //--- internal email mailbox name
   virtual LPCWSTR   Mailbox(void) const=0;
   virtual MTAPIRES  Mailbox(LPCWSTR mailbox)=0;
   //--- trade server id
   virtual UINT64    Server(void) const=0;
   //--- logs access limit EnManagerLimit
   virtual UINT      LimitLogs(void) const=0;
   virtual MTAPIRES  LimitLogs(const UINT limit)=0;
   //--- reports access limit EnManagerLimit
   virtual UINT      LimitReports(void) const=0;
   virtual MTAPIRES  LimitReports(const UINT limit)=0;
   //--- rights
   virtual UINT      Right(const UINT right) const=0;
   virtual MTAPIRES  Right(const UINT right,const UINT flags)=0;
   //--- allowed groups list
   virtual MTAPIRES  GroupAdd(LPCWSTR path)=0;
   virtual MTAPIRES  GroupUpdate(const UINT pos,LPCWSTR path)=0;
   virtual MTAPIRES  GroupShift(const UINT pos,const int shift)=0;
   virtual MTAPIRES  GroupDelete(const UINT pos)=0;
   virtual UINT      GroupTotal(void) const=0;
   virtual LPCWSTR   GroupNext(const UINT pos) const=0;
   //--- allowed addressed list
   virtual MTAPIRES  AccessAdd(IMTConManagerAccess* access)=0;
   virtual MTAPIRES  AccessUpdate(const UINT pos,const IMTConManagerAccess* access)=0;
   virtual MTAPIRES  AccessDelete(const UINT pos)=0;
   virtual MTAPIRES  AccessShift(const UINT pos,const int shift)=0;
   virtual UINT      AccessTotal(void) const=0;
   virtual MTAPIRES  AccessNext(const UINT pos,IMTConManagerAccess* access) const=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTConManager(void) {}
  };

class IMTConManagerSink
  {
public:
   virtual void      OnManagerAdd(const IMTConManager*    /*config*/) {  }
   virtual void      OnManagerUpdate(const IMTConManager* /*config*/) {  }
   virtual void      OnManagerDelete(const IMTConManager* /*config*/) {  }
   virtual void      OnManagerSync(void)                              {  }
   //--- config modification hooks (only for Server API)
   virtual MTAPIRES  HookManagerAdd(const UINT64 login,IMTConManager* /*new_cfg*/)                                 { return(MT_RET_OK); }
   virtual MTAPIRES  HookManagerUpdate(const UINT64 login,const IMTConManager* /*cfg*/,IMTConManager* /*new_cfg*/) { return(MT_RET_OK); }
   virtual MTAPIRES  HookManagerDelete(const UINT64 login,const IMTConManager* /*cfg*/)                            { return(MT_RET_OK); }
  };

class IMTConHistorySync
  {
public:
   //--- mode enumeration
   enum EnHistoryMode
     {
      HISTORY_DISABLED =0,
      HISTORY_ENABLED  =1,
      //--- enumeration borders
      HISTORY_FIRST    =HISTORY_DISABLED,
      HISTORY_LAST     =HISTORY_ENABLED,
     };
   //--- synchronization modex
   enum EnHistorySyncMode
     {
      MODE_REPLACE     =0, // syncronization with full previous data replace
      MODE_MERGE       =1, // syncronization with merge with previous data
      //--- enumeration borders
      MODE_FIRST       =MODE_REPLACE,
      MODE_LAST        =MODE_MERGE,
     };
   //--- server types
   enum EnHistorySyncServer
     {
      SERVER_MT4       =0, // MT4
      SERVER_MT5       =1, // MT5
      //--- enumeration borders
      SERVER_FIRST     =SERVER_MT4,
      SERVER_LAST      =SERVER_MT5,
     };
   //--- synchronization flags
   enum EnHistorySyncFlags
     {
      FLAG_SESSIONS     =1, // check quote session due synchronization
      FLAG_SYNONYMS     =2, // synchronize synonim symbols history
      //--- enumeration borders
      FLAG_NONE          =0,
      FLAG_ALL          =FLAG_SESSIONS|FLAG_SYNONYMS,
     };
   //--- synchronization data
   enum EnHistoryData
     {
      DATA_HISTORY_CHARTS=0, // charts only
      DATA_HISTORY_TICKS =1, // ticks only
      DATA_HISTORY_ALL   =2, // charts and ticks
      //--- enumeration borders
      DATA_HISTORY_FIRST =DATA_HISTORY_CHARTS,
      DATA_HISTORY_LAST  =DATA_HISTORY_ALL
     };
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTConHistorySync* param)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- server address
   virtual LPCWSTR   Server(void) const=0;
   virtual MTAPIRES  Server(LPCWSTR server)=0;
   //--- server type
   virtual UINT      ServerType(void) const=0;
   virtual MTAPIRES  ServerType(const UINT type)=0;
   //--- mode
   virtual UINT      Mode(void) const=0;
   virtual MTAPIRES  Mode(const UINT mode)=0;
   //--- synchronization mode
   virtual UINT      ModeSync(void) const=0;
   virtual MTAPIRES  ModeSync(const UINT type)=0;
   //--- time correction in minutes, 0 - autodetect
   virtual int       TimeCorrection(void) const=0;
   virtual MTAPIRES  TimeCorrection(const int correction)=0;
   //--- synchronized history start
   virtual INT64     From(void) const=0;
   virtual MTAPIRES  From(const INT64 from)=0;
   //--- synchronized history finish
   virtual INT64     To(void) const=0;
   virtual MTAPIRES  To(const INT64 to)=0;
   //--- synchronized symbols list
   virtual MTAPIRES  SymbolAdd(LPCWSTR path)=0;
   virtual MTAPIRES  SymbolUpdate(const UINT pos,LPCWSTR path)=0;
   virtual MTAPIRES  SymbolShift(const UINT pos,const int shift)=0;
   virtual MTAPIRES  SymbolDelete(const UINT pos)=0;
   virtual UINT      SymbolTotal(void) const=0;
   virtual LPCWSTR   SymbolNext(const UINT pos) const=0;
   //--- synchronization flags
   virtual UINT64    Flags(void) const=0;
   virtual MTAPIRES  Flags(const UINT64 flags)=0;
   //--- synchronization data
   virtual UINT      HistoryData(void) const=0;
   virtual MTAPIRES  HistoryData(const UINT data)=0;
   //--- login for MetaTrader 5 server
   virtual UINT64    Login(void) const=0;
   virtual MTAPIRES  Login(const UINT64 login)=0;
   //--- password for MetaTrader 5 server
   virtual LPCWSTR   Password(void) const=0;
   virtual MTAPIRES  Password(LPCWSTR password)=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTConHistorySync(void) {}
  };

class IMTConHistorySyncSink
  {
public:
   virtual void      OnHistorySyncAdd(const IMTConHistorySync*    /*config*/) {  }
   virtual void      OnHistorySyncUpdate(const IMTConHistorySync* /*config*/) {  }
   virtual void      OnHistorySyncDelete(const IMTConHistorySync* /*config*/) {  }
   virtual void      OnHistorySyncSync(void)                                  {  }
  };

class IMTConFeederTranslate
  {
public:
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTConFeederTranslate* param)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- symbol name in datafeed
   virtual LPCWSTR   Source(void) const=0;
   virtual MTAPIRES  Source(LPCWSTR source)=0;
   //--- symbol name in MT5
   virtual LPCWSTR   Symbol(void) const=0;
   virtual MTAPIRES  Symbol(LPCWSTR symbol)=0;
   //--- bid markup in points
   virtual INT       BidMarkup(void) const=0;
   virtual MTAPIRES  BidMarkup(const INT markup)=0;
   //--- ask markup in points
   virtual INT       AskMarkup(void) const=0;
   virtual MTAPIRES  AskMarkup(const INT markup)=0;
   //--- digits
   virtual UINT      Digits(void) const=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTConFeederTranslate(void) {}
  };

class IMTConFeederModule
  {
public:
   //--- necessary fields flags
   enum EnFeedersFieldFlags
     {
      FEED_FIELD_SERVER=1,            // server field
      FEED_FIELD_LOGIN =2,            // login field
      FEED_FIELD_PASS  =4,            // password field
      FEED_FIELD_PARAM =8,            // parameters
      //--- enumeration borders
      FEED_FIELD_NONE  =0,
      FEED_FIELD_ALL   =FEED_FIELD_SERVER|FEED_FIELD_LOGIN|FEED_FIELD_PASS|FEED_FIELD_PARAM
     };
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTConFeederModule* param)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- default datafeed name
   virtual LPCWSTR   Name(void) const=0;
   //--- vendor name
   virtual LPCWSTR   Vendor(void) const=0;
   //--- datafeed description
   virtual LPCWSTR   Description(void) const=0;
   //--- datafeed file name
   virtual LPCWSTR   Module(void) const=0;
   //--- default feed server address
   virtual LPCWSTR   FeedServer(void) const=0;
   //--- default feed server login
   virtual LPCWSTR   FeedLogin(void) const=0;
   //--- default feed server password
   virtual LPCWSTR   FeedPassword(void) const=0;
   //--- datafeed version
   virtual UINT      Version(void) const=0;
   //--- datafeed available modes (IMTConFeeder::EnFeedersFlags)
   virtual UINT      Modes(void) const=0;
   //--- changeable EnFeedersFieldFlags
   virtual UINT      Fields(void) const=0;
   //--- default datafeed parameters
   virtual UINT      ParameterTotal(void) const=0;
   virtual MTAPIRES  ParameterNext(const UINT pos,IMTConParam* param) const=0;
   virtual MTAPIRES  ParameterGet(LPCWSTR name,IMTConParam* param) const=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTConFeederModule(void) {}
  };

class IMTConGatewayTranslate
  {
public:
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTConGatewayTranslate* param)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- symbol name in datafeed
   virtual LPCWSTR   Source(void) const=0;
   virtual MTAPIRES  Source(LPCWSTR source)=0;
   //--- symbol name in MT5
   virtual LPCWSTR   Symbol(void) const=0;
   virtual MTAPIRES  Symbol(LPCWSTR symbol)=0;
   //--- bid markup in points
   virtual INT       BidMarkup(void) const=0;
   virtual MTAPIRES  BidMarkup(const INT markup)=0;
   //--- ask markup in points
   virtual INT       AskMarkup(void) const=0;
   virtual MTAPIRES  AskMarkup(const INT markup)=0;
   //--- digits
   virtual UINT      Digits(void) const=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTConGatewayTranslate(void) {}
  };

class IMTConGateway
  {
public:
   //--- gateway working flags
   enum EnGatewayFlags
     {
      GATEWAY_FLAG_REMOTE         =0x00000001,       // gateway works as remote service
      GATEWAY_FLAG_IMPORT_SYMBOLS =0x00000002,       // gateway can import symbols to MetaTrader platform
      GATEWAY_FLAG_IGNORE_QUOTES  =0x00000004,       // ignore quotes from gateway
      GATEWAY_FLAG_IMPORT_BALANCES=0x00000008,       // gateway can import client balances to MetaTrader platform
      GATEWAY_FLAG_EXTENDED_LOG   =0x00000010,       // gateway writes extended log
      GATEWAY_FLAG_SUPP_POSITIONS =0x00000020,       // gateway support request of trading positions from external trading system
      GATEWAY_FLAG_PROFILLING     =0x00000040,       // gateway support request of trading positions from external trading system
      GATEWAY_FLAG_TRIAL          =0x00000080,       // gateway in trial mode
      //--- flags borders
      GATEWAY_FLAG_NONE           =0,
      GATEWAY_FLAG_ALL            =GATEWAY_FLAG_REMOTE|GATEWAY_FLAG_IMPORT_SYMBOLS|
      GATEWAY_FLAG_IGNORE_QUOTES| GATEWAY_FLAG_IMPORT_BALANCES| GATEWAY_FLAG_EXTENDED_LOG | GATEWAY_FLAG_SUPP_POSITIONS |
      GATEWAY_FLAG_PROFILLING | GATEWAY_FLAG_TRIAL
     };
   //--- gateway modes
   enum EnGatewayMode
     {
      GATEWAY_DISABLED  =0,
      GATEWAY_ENABLED   =1,
      //--- enumeration borders
      GATEWAY_FIRST     =GATEWAY_DISABLED,
      GATEWAY_LAST      =GATEWAY_ENABLED
     };
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTConGateway* param)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- gateway name
   virtual LPCWSTR   Name(void) const=0;
   virtual MTAPIRES  Name(LPCWSTR name)=0;
   //--- gateway trade request routing id (dealer id)
   virtual UINT64    ID(void) const=0;
   virtual MTAPIRES  ID(const UINT64 id)=0;
   //--- gateway module name
   virtual LPCWSTR   Module(void) const=0;
   virtual MTAPIRES  Module(LPCWSTR name)=0;
   //--- gateway server address for trading
   virtual LPCWSTR   TradingServer(void) const=0;
   virtual MTAPIRES  TradingServer(LPCWSTR server)=0;
   //--- gateway login for trading
   virtual LPCWSTR   TradingLogin(void) const=0;
   virtual MTAPIRES  TradingLogin(LPCWSTR login)=0;
   //--- gateway password for trading
   virtual LPCWSTR   TradingPassword(void) const=0;
   virtual MTAPIRES  TradingPassword(LPCWSTR password)=0;
   //--- gateway server address
   virtual LPCWSTR   GatewayServer(void) const=0;
   virtual MTAPIRES  GatewayServer(LPCWSTR server)=0;
   //--- EnGatewayMode
   virtual UINT      Mode(void) const=0;
   virtual MTAPIRES  Mode(const UINT mode)=0;
   //--- EnGatewayFlags
   virtual UINT      Flags(void) const=0;
   virtual MTAPIRES  Flags(const UINT flags)=0;
   //--- obsolete value
   virtual UINT      ObsoleteValue(void) const=0;
   virtual MTAPIRES  ObsoleteValue(const UINT value)=0;
   //--- gateway reconnect timeout
   virtual UINT      TimeoutReconnect(void) const=0;
   virtual MTAPIRES  TimeoutReconnect(const UINT timeout)=0;
   //--- gateway sleep timeout
   virtual UINT      TimeoutSleep(void) const=0;
   virtual MTAPIRES  TimeoutSleep(const UINT timeout)=0;
   //--- gateway attempts before timeout
   virtual UINT      TimeoutAttempts(void) const=0;
   virtual MTAPIRES  TimeoutAttempts(const UINT attempts)=0;
   //--- gateway additional parameters access
   virtual MTAPIRES  ParameterAdd(IMTConParam* param)=0;
   virtual MTAPIRES  ParameterUpdate(const UINT pos,const IMTConParam* param)=0;
   virtual MTAPIRES  ParameterDelete(const UINT pos)=0;
   virtual MTAPIRES  ParameterClear(void)=0;
   virtual MTAPIRES  ParameterShift(const UINT pos,const int shift)=0;
   virtual UINT      ParameterTotal(void) const=0;
   virtual MTAPIRES  ParameterNext(const UINT pos,IMTConParam* param) const=0;
   virtual MTAPIRES  ParameterGet(LPCWSTR name,IMTConParam* param) const=0;
   //--- list of symbols for translating quotes
   virtual MTAPIRES  SymbolAdd(LPCWSTR path)=0;
   virtual MTAPIRES  SymbolUpdate(const UINT pos,LPCWSTR path)=0;
   virtual MTAPIRES  SymbolDelete(const UINT pos)=0;
   virtual MTAPIRES  SymbolClear(void)=0;
   virtual MTAPIRES  SymbolShift(const UINT pos,const int shift)=0;
   virtual UINT      SymbolTotal(void) const=0;
   virtual LPCWSTR   SymbolNext(const UINT pos) const=0;
   //--- list of users groups who works through gateway
   virtual MTAPIRES  GroupAdd(LPCWSTR path)=0;
   virtual MTAPIRES  GroupUpdate(const UINT pos,LPCWSTR path)=0;
   virtual MTAPIRES  GroupDelete(const UINT pos)=0;
   virtual MTAPIRES  GroupClear(void)=0;
   virtual MTAPIRES  GroupShift(const UINT pos,const int shift)=0;
   virtual UINT      GroupTotal(void) const=0;
   virtual LPCWSTR   GroupNext(const UINT pos) const=0;
   //--- list of symbols translations
   virtual MTAPIRES  TranslateAdd(IMTConGatewayTranslate* param)=0;
   virtual MTAPIRES  TranslateUpdate(const UINT pos,const IMTConGatewayTranslate* param)=0;
   virtual MTAPIRES  TranslateDelete(const UINT pos)=0;
   virtual MTAPIRES  TranslateClear(void)=0;
   virtual MTAPIRES  TranslateShift(const UINT pos,const int shift)=0;
   virtual UINT      TranslateTotal(void) const=0;
   virtual MTAPIRES  TranslateNext(const UINT pos,IMTConGatewayTranslate* param) const=0;
   virtual MTAPIRES  TranslateGet(LPCWSTR symbol,IMTConGatewayTranslate* param) const=0;
   //--- gateway server login
   virtual UINT64    GatewayLogin(void) const=0;
   virtual MTAPIRES  GatewayLogin(UINT64 login)=0;
   //--- gateway server password
   virtual LPCWSTR   GatewayPassword(void) const=0;
   virtual MTAPIRES  GatewayPassword(LPCWSTR password)=0;
   //--- list of symbols translations
   virtual MTAPIRES  TranslateGetSource(LPCWSTR source,IMTConGatewayTranslate* param) const=0;
   //--- gateway given name
   virtual LPCWSTR   Gateway(void) const=0;
   //--- gateway state information
   virtual bool      StateConnected(void) const=0;
   virtual UINT      StateReceivedTicks(void) const=0;
   virtual UINT      StateReceivedBooks(void) const=0;
   virtual UINT      StateTrafficIn(void) const=0;
   virtual UINT      StateTrafficOut(void) const=0;
   virtual UINT      StateTradesTotal(void) const=0;
   virtual UINT      StateTradesAverageTime(void) const=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTConGateway(void) {}
  };

class IMTConGatewayModule
  {
   //--- changeable fields flags
   enum EnGatewayFieldMask
     {
      GATEWAY_FIELD_SERVER=1,         // server field
      GATEWAY_FIELD_LOGIN =2,         // login field
      GATEWAY_FIELD_PASS  =4,         // password field
      GATEWAY_FIELD_PARAM =8,         // parameters
      //--- enumeration borders
      GATEWAY_FIELD_NONE  =0,
      GATEWAY_FIELD_ALL   =GATEWAY_FIELD_SERVER|GATEWAY_FIELD_LOGIN|GATEWAY_FIELD_PASS|GATEWAY_FIELD_PARAM
     };
public:
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTConGatewayModule* param)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- default gateway name
   virtual LPCWSTR   Name(void) const=0;
   //--- vendor name
   virtual LPCWSTR   Vendor(void) const=0;
   //--- gateway description
   virtual LPCWSTR   Description(void) const=0;
   //--- gateway file name
   virtual LPCWSTR   Module(void) const=0;
   //--- gateway default server address for trading
   virtual LPCWSTR   TradingServer(void) const=0;
   //--- gateway default login for trading
   virtual LPCWSTR   TradingLogin(void) const=0;
   //--- gateway default password for trading
   virtual LPCWSTR   TradingPassword(void) const=0;
   //--- gateway version
   virtual UINT      Version(void) const=0;
   //--- gateway available flags (IMTConGateway::EnGatewayFlags)
   virtual UINT      Flags(void) const=0;
   //--- changeable EnFeedersFieldFlags
   virtual UINT      Fields(void) const=0;
   //--- default gateway parameters
   virtual UINT      ParameterTotal(void) const=0;
   virtual MTAPIRES  ParameterNext(const UINT pos,IMTConParam* param) const=0;
   virtual MTAPIRES  ParameterGet(LPCWSTR name,IMTConParam* param) const=0;
   //--- gateway given name
   virtual LPCWSTR   Gateway(void) const=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTConGatewayModule(void) {}
  };

class IMTConFeeder
  {
public:
   //--- datafeed working flags
   enum EnFeedersFlags
     {
      FEED_FLAG_QUOTES =0x0000001,            // feeder should send quotes
      FEED_FLAG_NEWS   =0x0000002,            // feeder should send news
      FEED_FLAG_REMOTE =0x0000008,            // feeder works as remote service
      FEED_FLAG_TRIAL  =0x0000010,            // feeder works in trial mode
      //--- flags borders
      FEED_FLAG_NONE   =0,
      FEED_FLAG_ALL    =FEED_FLAG_QUOTES|FEED_FLAG_NEWS|FEED_FLAG_REMOTE|FEED_FLAG_TRIAL
     };
   //--- datafeed working modes
   enum EnFeedersMode
     {
      FEEDER_DISABLED  =0,
      FEEDER_ENABLED   =1,
      //--- enumeration borders
      FEEDER_FIRST     =FEEDER_DISABLED,
      FEEDER_LAST      =FEEDER_ENABLED
     };
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTConFeeder* param)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- datafeed name
   virtual LPCWSTR   Name(void) const=0;
   virtual MTAPIRES  Name(LPCWSTR name)=0;
   //--- datafeed module filename
   virtual LPCWSTR   Module(void) const=0;
   virtual MTAPIRES  Module(LPCWSTR name)=0;
   //--- feed server address
   virtual LPCWSTR   FeedServer(void) const=0;
   virtual MTAPIRES  FeedServer(LPCWSTR server)=0;
   //--- feed server login
   virtual LPCWSTR   FeedLogin(void) const=0;
   virtual MTAPIRES  FeedLogin(LPCWSTR login)=0;
   //--- feed server password
   virtual LPCWSTR   FeedPassword(void) const=0;
   virtual MTAPIRES  FeedPassword(LPCWSTR password)=0;
   //--- EnFeedersMode
   virtual UINT      Mode(void) const=0;
   virtual MTAPIRES  Mode(const UINT mode)=0;
   //--- EnFeedersFlags
   virtual UINT      Flags(void) const=0;
   virtual MTAPIRES  Flags(const UINT flags)=0;
   //--- comma separated news keywords
   virtual LPCWSTR   Keywords(void) const=0;
   virtual MTAPIRES  Keywords(LPCWSTR keywords)=0;
   //--- news category
   virtual LPCWSTR   Categories(void) const=0;
   virtual MTAPIRES  Categories(LPCWSTR categories)=0;
   //--- obsolete value
   virtual UINT      ObsoleteValue(void) const=0;
   virtual MTAPIRES  ObsoleteValue(const UINT value)=0;
   //--- datafeed reconnect timeout
   virtual UINT      TimeoutReconnect(void) const=0;
   virtual MTAPIRES  TimeoutReconnect(const UINT timeout)=0;
   //--- datafeed sleep timeout
   virtual UINT      TimeoutSleep(void) const=0;
   virtual MTAPIRES  TimeoutSleep(const UINT timeout)=0;
   //--- reconnect attempts before timeout
   virtual UINT      TimeoutAttempts(void) const=0;
   virtual MTAPIRES  TimeoutAttempts(const UINT attempts)=0;
   //--- datafeed additional parameters access
   virtual MTAPIRES  ParameterAdd(IMTConParam* param)=0;
   virtual MTAPIRES  ParameterUpdate(const UINT pos,const IMTConParam* param)=0;
   virtual MTAPIRES  ParameterDelete(const UINT pos)=0;
   virtual MTAPIRES  ParameterClear(void)=0;
   virtual MTAPIRES  ParameterShift(const UINT pos,const int shift)=0;
   virtual UINT      ParameterTotal(void) const=0;
   virtual MTAPIRES  ParameterNext(const UINT pos,IMTConParam* param) const=0;
   virtual MTAPIRES  ParameterGet(LPCWSTR name,IMTConParam* param) const=0;
   //--- list of symbols for translating quotes
   virtual MTAPIRES  SymbolAdd(LPCWSTR path)=0;
   virtual MTAPIRES  SymbolUpdate(const UINT pos,LPCWSTR path)=0;
   virtual MTAPIRES  SymbolDelete(const UINT pos)=0;
   virtual MTAPIRES  SymbolClear(void)=0;
   virtual MTAPIRES  SymbolShift(const UINT pos,const int shift)=0;
   virtual UINT      SymbolTotal(void) const=0;
   virtual LPCWSTR   SymbolNext(const UINT pos) const=0;
   //--- list of symbols translations
   virtual MTAPIRES  TranslateAdd(IMTConFeederTranslate* param)=0;
   virtual MTAPIRES  TranslateUpdate(const UINT pos,const IMTConFeederTranslate* param)=0;
   virtual MTAPIRES  TranslateDelete(const UINT pos)=0;
   virtual MTAPIRES  TranslateClear(void)=0;
   virtual MTAPIRES  TranslateShift(const UINT pos,const int shift)=0;
   virtual UINT      TranslateTotal(void) const=0;
   virtual MTAPIRES  TranslateNext(const UINT pos,IMTConFeederTranslate* param) const=0;
   virtual MTAPIRES  TranslateGet(LPCWSTR symbol,IMTConFeederTranslate* param) const=0;
   //--- gateway server address
   virtual LPCWSTR   GatewayServer(void) const=0;
   virtual MTAPIRES  GatewayServer(LPCWSTR server)=0;
   //--- gateway server login
   virtual UINT64    GatewayLogin(void) const=0;
   virtual MTAPIRES  GatewayLogin(UINT64 login)=0;
   //--- gateway server password
   virtual LPCWSTR   GatewayPassword(void) const=0;
   virtual MTAPIRES  GatewayPassword(LPCWSTR password)=0;
   //--- feeder state information
   virtual bool      StateConnected(void) const=0;
   virtual UINT      StateReceivedTicks(void) const=0;
   virtual UINT      StateReceivedBooks(void) const=0;
   virtual UINT      StateReceivedNews(void) const=0;
   virtual UINT      StateTrafficIn(void) const=0;
   virtual UINT      StateTrafficOut(void) const=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTConFeeder(void) {}
  };

class IMTConFeederSink
  {
public:
   virtual void      OnFeederAdd(const IMTConFeeder*    /*feeder*/) {  }
   virtual void      OnFeederUpdate(const IMTConFeeder* /*feeder*/) {  }
   virtual void      OnFeederDelete(const IMTConFeeder* /*feeder*/) {  }
   virtual void      OnFeederSync(void)                             {  }
  };

class IMTConGatewaySink
  {
public:
   virtual void      OnGatewayAdd(const IMTConGateway*    /*gateway*/) {  }
   virtual void      OnGatewayUpdate(const IMTConGateway* /*gateway*/) {  }
   virtual void      OnGatewayDelete(const IMTConGateway* /*gateway*/) {  }
   virtual void      OnGatewaySync(void)                               {  }
  };

class IMTConReportModule
  {
public:
   //--- snapshot mode flags
   enum EnSnapshots
     {
      SNAPSHOT_NONE          =0x0,       // without snapshots
      SNAPSHOT_USERS         =0x1,       // users database snapshot for request
      SNAPSHOT_USERS_FULL    =0x2,       // full users database snapshot
      SNAPSHOT_ACCOUNTS      =0x4,       // trade account states snapshot
      SNAPSHOT_ACCOUNTS_FULL =0x8,       // trade account states snapshot for request
      SNAPSHOT_ORDERS        =0x10,      // orders database snapshot
      SNAPSHOT_ORDERS_FULL   =0x20,      // orders database snapshot for request
      SNAPSHOT_POSITIONS     =0x40,      // positions database snapshot
      SNAPSHOT_POSITIONS_FULL=0x80,      // positions database snapshot for request
      //---
      SNAPSHOT_FIRST         =SNAPSHOT_NONE,
      SNAPSHOT_LAST          =SNAPSHOT_POSITIONS_FULL,
     };
   //--- types reports
   enum EnTypes
     {
      TYPE_NONE              =0x0,        // no support
      TYPE_HTML              =0x1,        // HTML
      TYPE_TABLE             =0x2,        // binary table
      //---
      TYPE_FIRST             =TYPE_NONE,
      TYPE_LAST              =TYPE_TABLE,
      TYPE_ALL               =TYPE_HTML|TYPE_TABLE
     };
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTConReportModule* param)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- default report name
   virtual LPCWSTR   Name(void) const=0;
   //--- vendor name
   virtual LPCWSTR   Vendor(void) const=0;
   //--- report description
   virtual LPCWSTR   Description(void) const=0;
   //--- report file name
   virtual LPCWSTR   Module(void) const=0;
   //--- report index in file
   virtual UINT      Index(void) const=0;
   //--- MT5 server-owner id
   virtual UINT64    Server(void) const=0;
   //--- plugin version
   virtual UINT      Version(void) const=0;
   //--- plugin Server API version
   virtual UINT      VersionAPI(void) const=0;
   //--- neccessary Internet Explorer version
   virtual UINT      VersionIE(void) const=0;
   //--- available types
   virtual UINT      Types(void) const=0;
   //--- neccessary data snapshots
   virtual UINT      Snapshots(void) const=0;
   //--- report default parameters
   virtual UINT      ParameterTotal(void) const=0;
   virtual MTAPIRES  ParameterNext(const UINT pos,IMTConParam* param) const=0;
   virtual MTAPIRES  ParameterGet(LPCWSTR name,IMTConParam* param) const=0;
   //--- report request input params
   virtual UINT      InputTotal(void) const=0;
   virtual MTAPIRES  InputNext(const UINT pos,IMTConParam* param) const=0;
   virtual MTAPIRES  InputGet(LPCWSTR name,IMTConParam* param) const=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTConReportModule(void) {}
  };

class IMTConReport
  {
public:
   //--- report mode
   enum EnReportMode
     {
      REPORT_DISABLED=0,
      REPORT_ENABLED =1,
      //--- enumeration borders
      REPORT_FIRST   =REPORT_DISABLED,
      REPORT_LAST    =REPORT_ENABLED,
     };
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTConReport* param)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- plugin name
   virtual LPCWSTR   Name(void) const=0;
   virtual MTAPIRES  Name(LPCWSTR name)=0;
   //--- MT5 server ID
   virtual UINT64    Server(void) const=0;
   virtual MTAPIRES  Server(const UINT64 server)=0;
   //--- plugin report name
   virtual LPCWSTR   Module(void) const=0;
   virtual MTAPIRES  Module(LPCWSTR name)=0;
   //--- plugin mode
   virtual UINT      Mode(void) const=0;
   virtual MTAPIRES  Mode(const UINT mode)=0;
   //--- plugin parameters
   virtual MTAPIRES  ParameterAdd(IMTConParam* param)=0;
   virtual MTAPIRES  ParameterUpdate(const UINT pos,const IMTConParam* param)=0;
   virtual MTAPIRES  ParameterDelete(const UINT pos)=0;
   virtual MTAPIRES  ParameterClear(void)=0;
   virtual MTAPIRES  ParameterShift(const UINT pos,const int shift)=0;
   virtual UINT      ParameterTotal(void) const=0;
   virtual MTAPIRES  ParameterNext(const UINT pos,IMTConParam* param) const=0;
   virtual MTAPIRES  ParameterGet(LPCWSTR name,IMTConParam* param) const=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTConReport(void) {}
  };

class IMTConReportSink
  {
public:
   virtual void      OnReportAdd(const IMTConReport*    /*report*/) {  }
   virtual void      OnReportUpdate(const IMTConReport* /*report*/) {  }
   virtual void      OnReportDelete(const IMTConReport* /*report*/) {  }
   virtual void      OnReportSync(void)                             {  }
  };

class IMTConCondition
  {
public:
   //--- condition types
   enum EnRouteCondition
     {
      //--- trade request parameters
      CONDITION_DATETIME            =0,       // datetime
      CONDITION_SYMBOL              =1,       // symbol
      CONDITION_VOLUME              =2,       // volume
      CONDITION_MARKET_DEVIATION    =3,       // deviation from market
      CONDITION_TIME                =4,       // time (in minutes from 00:00)
      CONDITION_WEEKDAY             =5,       // day of week 0 - sunday
      CONDITION_COMMENT             =6,       // find substring in request comment
      CONDITION_EXPERT              =7,       // request placed by expert
      CONDITION_SIGNAL              =8,       // request placed by signal
      CONDITION_DEALER_LOGIN        =9,       // dealer processed specified order or position
      CONDITION_SOURCE_LOGIN        =10,      // dealer processed specified order or position
      CONDITION_MARKET_DEVIATION_SPR=11,      // deviation from market in spreads
      CONDITION_GAP                 =12,      // symbol in gap mode
      CONDITION_PRICE               =13,      // price in request
      //--- client data
      CONDITION_LOGIN               =1000,    // login
      CONDITION_GROUP               =1001,    // group
      CONDITION_COUNTRY             =1002,    // country
      CONDITION_CITY                =1003,    // city
      CONDITION_COLOR               =1004,    // color
      CONDITION_LEVERAGE            =1005,    // leverage
      CONDITION_COMMENT_CLIENT      =1006,    // client record comment
      CONDITION_ZIPCODE             =1007,    // ZIP code
      //--- client trade account
      CONDITION_MARGIN              =2000,    // margin
      CONDITION_MARGIN_LEVEL        =2001,    // margin level
      CONDITION_MARGIN_FREE         =2002,    // free margin
      CONDITION_EQUITY              =2003,    // equity
      CONDITION_BALANCE             =2004,    // balance
      CONDITION_PROFIT              =2005,    // profit
      //--- trading statistic
      CONDITION_DAILY_DEALS         =3000,    // deals count
      CONDITION_DAILY_DEALS_PERIOD  =3001,    // average period between deals
      CONDITION_DAILY_PROFIT        =3002,    // fixed profit
      //--- position and orders parameters
      CONDITION_POSITION_VOLUME     =4000,    // position volume
      CONDITION_POSITION_PROFIT     =4001,    // position profit
      CONDITION_POSITION_AGE        =4002,    // position age
      CONDITION_POSITION_MODIFY_TIME=4003,    // modify last time
      CONDITION_POSITION_AVERAGE_TIME=4004,   // position average time
      CONDITION_POSITION_TOTAL       =4005,   // total client positions
      CONDITION_POSITION_TOTAL_SYMBOL=4006,   // client positions by request symbol
      CONDITION_ORDER_TOTAL          =4007,   // total client orders
      CONDITION_ORDER_TOTAL_SYMBOL   =4008,   // client orders by request symbol
      CONDITION_POSITION_SL_TOUCHED  =4009,   // position SL touched
      CONDITION_POSITION_TP_TOUCHED  =4010,   // position TP touched
      CONDITION_ORDER_SL_TOUCHED     =4011,   // order SL touched
      CONDITION_ORDER_TP_TOUCHED     =4012,   // order TP touched
      CONDITION_ORDER_ENTRY_IN       =4013,   // order entry in
      CONDITION_ORDER_ENTRY_OUT      =4014,   // order entry out
      //--- trade symbol parameters
      CONDITION_SYMBOL_SPREAD        =5000,   // ������� ����� �� �����������
      //--- enumeration borders
      CONDITION_FIRST                =CONDITION_DATETIME,
      CONDITION_LAST                 =CONDITION_SYMBOL_SPREAD
     };
   //--- compare method
   enum EnConditionRule
     {
      RULE_EQ                       =0,       // equal      (==)
      RULE_NOT_EQ                   =1,       // not equal  (!=) 
      RULE_GREATER                  =2,       // greater    (> )
      RULE_NOT_LESS                 =3,       // not less   (>=)
      RULE_LESS                     =4,       // less       (< )
      RULE_NOT_GREATER              =5,       // not greater(<=)
      //--- enumeration borders
      RULE_FIRST                    =RULE_EQ,
      RULE_LAST                     =RULE_NOT_GREATER
     };
   //--- condition value type
   enum EnConditionType
     {
      TYPE_NONE                     =0,       // none
      TYPE_STRING                   =1,       // string
      TYPE_INT                      =2,       // integer
      TYPE_UINT                     =3,       // unsigned integer
      TYPE_DOUBLE                   =4,       // double
      TYPE_COLOR                    =5,       // color
      TYPE_MONEY                    =6,       // money
      TYPE_VOLUME                   =7,       // volume
      TYPE_DATETIME                 =8,       // datetime
      TYPE_LEVERAGE                 =9,       // leverage
      TYPE_BOOL                     =10,      // boolean
      TYPE_TIME                     =11,      // time
      TYPE_WEEKDAY                  =12,      // weekday
      //--- enumeration borders
      TYPE_FIRST                    =TYPE_NONE,
      TYPE_LAST                     =TYPE_WEEKDAY
     };
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTConCondition* config)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- condition
   virtual UINT      Condition(void) const=0;
   virtual MTAPIRES  Condition(const UINT condition)=0;
   //--- rule
   virtual UINT      Rule(void) const=0;
   virtual MTAPIRES  Rule(const UINT rule)=0;
   //--- type
   virtual UINT      ValueType(void) const=0;
   //--- value int
   virtual INT64     ValueInt(void) const=0;
   virtual MTAPIRES  ValueInt(const INT64 value)=0;
   //--- value uint
   virtual UINT64    ValueUInt(void) const=0;
   virtual MTAPIRES  ValueUInt(const UINT64 value)=0;
   //--- value double
   virtual double    ValueDouble(void) const=0;
   virtual MTAPIRES  ValueDouble(const double value)=0;
   //--- value string
   virtual LPCWSTR   ValueString(void) const=0;
   virtual MTAPIRES  ValueString(LPCWSTR value)=0;
   //--- value color
   virtual COLORREF  ValueColor(void) const=0;
   virtual MTAPIRES  ValueColor(const COLORREF value)=0;
   //--- value money
   virtual double    ValueMoney(void) const=0;
   virtual MTAPIRES  ValueMoney(const double value)=0;
   //--- value volume
   virtual UINT64    ValueVolume(void) const=0;
   virtual MTAPIRES  ValueVolume(const UINT64 value)=0;
   //--- value datetime
   virtual INT64     ValueDatetime(void) const=0;
   virtual MTAPIRES  ValueDatetime(const INT64 value)=0;
   //--- value leverage
   virtual INT64     ValueLeverage(void) const=0;
   virtual MTAPIRES  ValueLeverage(const INT64 value)=0;
   //--- value bool
   virtual bool      ValueBool(void) const=0;
   virtual MTAPIRES  ValueBool(const bool value)=0;
   //--- value time
   virtual UINT      ValueTime(void) const=0;
   virtual MTAPIRES  ValueTime(const UINT value)=0;
   //--- value weekday
   virtual UINT      ValueWeekDay(void) const=0;
   virtual MTAPIRES  ValueWeekDay(const UINT value)=0;
   //--- value volume with extended accuracy
   virtual UINT64    ValueVolumeExt(void) const=0;
   virtual MTAPIRES  ValueVolumeExt(const UINT64 value)=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTConCondition(void) {}
  };

class IMTConRouteDealer
  {
public:
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTConRouteDealer* config)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- login
   virtual UINT64    Login(void) const=0;
   virtual MTAPIRES  Login(const UINT64 login)=0;
   //--- name
   virtual LPCWSTR   Name(void) const=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTConRouteDealer(void) {}
  };

class IMTConRoute
  {
public:
   //--- modes
   enum EnRouteMode
     {
      MODE_DISABLED                 =0,
      MODE_ENABLED                  =1,
      //--- enumeration borders
      MODE_FIRST                    =MODE_DISABLED,
      MODE_LAST                     =MODE_ENABLED
     };
   //--- trade request flags
   enum EnRouteFlags
     {
      REQUEST_NONE                  =0x00000000,  // empty
      //--- client requests
      REQUEST_PRICE                 =0x00000001,  // price request
      REQUEST_REQUEST               =0x00000002,  // market order on request execution
      REQUEST_INSTANT               =0x00000004,  // market order on instant execution
      REQUEST_MARKET                =0x00000008,  // market order on market execution
      REQUEST_EXCHANGE              =0x00000010,  // market order on exchange execution
      REQUEST_PENDING               =0x00000020,  // pending order place
      REQUEST_SLTP                  =0x00000040,  // SL/TP modification
      REQUEST_MODIFY                =0x00000080,  // pending order modification
      REQUEST_REMOVE                =0x00000100,  // pending order modification
      //--- activations
      REQUEST_ACTIVATE              =0x00000200,  // pending order activation
      REQUEST_STOPLIMIT             =0x00000400,  // Stop-Limit order activation
      REQUEST_SL                    =0x00000800,  // SL activation
      REQUEST_TP                    =0x00001000,  // TP activation
      REQUEST_STOPOUT_ORDER         =0x00002000,  // order Stop-Out
      REQUEST_STOPOUT_POSITION      =0x00004000,  // position Stop-Out
      REQUEST_EXPIRATION            =0x00008000,  // order expiration
      //--- dealer request
      REQUEST_DEALER_POS_EXECUTE    =0x00010000,  // position placing by dealer
      REQUEST_DEALER_ORD_PENDING    =0x00020000,  // order placing by dealer
      REQUEST_DEALER_POS_MODIFY     =0x00040000,  // position modification by dealer
      REQUEST_DEALER_ORD_MODIFY     =0x00080000,  // order modification by dealer
      REQUEST_DEALER_ORD_REMOVE     =0x00100000,  // order remove  by dealer
      REQUEST_DEALER_ORD_ACTIVATE   =0x00200000,  // order actvation by dealer
      REQUEST_DEALER_ORD_SLIMIT     =0x00400000,  // Stop-Limit order  actvation by dealer
      REQUEST_DEALER_CLOSE_BY       =0x00800000,  // close by hedged position by dealer
      //--- client requests
      REQUEST_CLOSE_BY              =0x01000000,  // close by hedged position
      //--- all
      REQUEST_ALL                   =REQUEST_PRICE | REQUEST_REQUEST | REQUEST_INSTANT | REQUEST_MARKET |
      REQUEST_EXCHANGE   | REQUEST_PENDING         | REQUEST_SLTP    | REQUEST_MODIFY  | REQUEST_REMOVE |
      REQUEST_ACTIVATE   | REQUEST_STOPLIMIT       | REQUEST_SL      | REQUEST_TP      | REQUEST_STOPOUT_ORDER |
      REQUEST_STOPOUT_POSITION | REQUEST_EXPIRATION| REQUEST_DEALER_POS_EXECUTE | REQUEST_DEALER_ORD_PENDING   |
      REQUEST_DEALER_POS_MODIFY|REQUEST_DEALER_ORD_MODIFY | REQUEST_DEALER_ORD_REMOVE | REQUEST_DEALER_ORD_ACTIVATE | REQUEST_DEALER_ORD_SLIMIT | REQUEST_DEALER_CLOSE_BY |
      REQUEST_CLOSE_BY,
      //--- enumeration borders
      REQUEST_FIRST                 =REQUEST_NONE,
      REQUEST_LAST                  =REQUEST_ALL
     };
   //--- order-position types
   enum EnTypeFlags
     {
      TYPE_NONE                     =0x0000,  // none
      TYPE_BUY                      =0x0001,  // BUY
      TYPE_SELL                     =0x0002,  // SELL
      TYPE_BUY_LIMIT                =0x0004,  // BUY LIMIT
      TYPE_SELL_LIMIT               =0x0008,  // SELL LIMIT
      TYPE_BUY_STOP                 =0x0010,  // BUY STOP
      TYPE_SELL_STOP                =0x0020,  // SELL STOP
      TYPE_BUY_STOP_LIMIT           =0x0040,  // BUY STOP LIMIT
      TYPE_SELL_STOP_LIMIT          =0x0080,  // SELL STOP LIMIT
      //--- all
      TYPE_ALL                      =TYPE_BUY|TYPE_SELL|TYPE_BUY_LIMIT|TYPE_SELL_LIMIT|
      TYPE_BUY_STOP|TYPE_SELL_STOP|TYPE_BUY_STOP_LIMIT|TYPE_SELL_STOP_LIMIT,
      //--- enumeration borders
      TYPE_FIRST                    =TYPE_NONE,
      TYPE_LAST                     =TYPE_ALL
     };
   //--- actions
   enum EnRouteAction
     {
      //--- intermediate actions
      ACTION_DELAY_TIME             =0,       // N seconds timeout
      ACTION_DELAY_TICK             =1,       // N ticks timeout
      ACTION_CLEAR_TP               =2,       // clear TP
      ACTION_CLEAR_SL               =3,       // clear SL
      ACTION_CLEAR_SLTP             =4,       // clear SL & TP
      //--- final actions
      ACTION_DEALER                 =1001,    // route request to dealers\gateways
      ACTION_DEALER_ONLINE          =1002,    // route request to online dealers\gateways
      ACTION_REJECT                 =1003,    // reject request
      ACTION_REQUOTE                =1004,    // requote request using current market
      ACTION_CONFIRM_CLIENT         =1005,    // confirm using client price
      ACTION_CONFIRM_MARKET         =1006,    // confirm using market price
      ACTION_CANCEL_ORDER           =1007,    // reject request and cancel order
      //--- enumeration borders
      ACTION_FIRST                  =ACTION_DELAY_TIME,
      ACTION_LAST                   =ACTION_CANCEL_ORDER
     };
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTConRoute* config)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- name
   virtual LPCWSTR   Name(void) const=0;
   virtual MTAPIRES  Name(LPCWSTR name)=0;
   //--- request EnRouteMode
   virtual UINT      Mode(void) const=0;
   virtual MTAPIRES  Mode(const UINT mode)=0;
   //--- request EnRouteFlags
   virtual UINT      Request(void) const=0;
   virtual MTAPIRES  Request(const UINT request)=0;
   //--- order-posiion type EnRouteType
   virtual UINT      Type(void) const=0;
   virtual MTAPIRES  Type(const UINT type)=0;
   //--- action for request EnRouteAction
   virtual UINT      Action(void) const=0;
   virtual MTAPIRES  Action(const UINT action)=0;
   //--- action value type IMTConRouteCondition::EnConditionType
   virtual UINT      ParamType(void) const=0;
   //--- value int
   virtual INT64     ParamInt(void) const=0;
   virtual MTAPIRES  ParamInt(const INT64 value)=0;
   //--- value uind
   virtual UINT64    ParamUInt(void) const=0;
   virtual MTAPIRES  ParamUInt(const UINT64 value)=0;
   //--- value double
   virtual double    ParamDouble(void) const=0;
   virtual MTAPIRES  ParamDouble(const double value)=0;
   //--- value string
   virtual LPCWSTR   ParamString(void) const=0;
   virtual MTAPIRES  ParamString(LPCWSTR value)=0;
   //--- value color
   virtual COLORREF  ParamColor(void) const=0;
   virtual MTAPIRES  ParamColor(const COLORREF value)=0;
   //--- value money
   virtual double    ParamMoney(void) const=0;
   virtual MTAPIRES  ParamMoney(const double value)=0;
   //--- value volume
   virtual UINT64    ParamVolume(void) const=0;
   virtual MTAPIRES  ParamVolume(const UINT64 value)=0;
   //--- value datetime
   virtual INT64     ParamDatetime(void) const=0;
   virtual MTAPIRES  ParamDatetime(const INT64 value)=0;
   //--- value leverage
   virtual INT64     ParamLeverage(void) const=0;
   virtual MTAPIRES  ParamLeverage(const INT64 value)=0;
   //--- value bool
   virtual bool      ParamBool(void) const=0;
   virtual MTAPIRES  ParamBool(const bool value)=0;
   //--- value time
   virtual UINT      ParamTime(void) const=0;
   virtual MTAPIRES  ParamTime(const UINT value)=0;
   //--- conditions
   virtual MTAPIRES  ConditionAdd(IMTConCondition* condition)=0;
   virtual MTAPIRES  ConditionUpdate(const UINT pos,const IMTConCondition* condition)=0;
   virtual MTAPIRES  ConditionDelete(const UINT pos)=0;
   virtual MTAPIRES  ConditionClear(void)=0;
   virtual MTAPIRES  ConditionShift(const UINT pos,const int shift)=0;
   virtual UINT      ConditionTotal(void) const=0;
   virtual MTAPIRES  ConditionNext(const UINT pos,IMTConCondition* condition) const=0;
   //--- dealers
   virtual MTAPIRES  DealerAdd(IMTConRouteDealer* dealer)=0;
   virtual MTAPIRES  DealerUpdate(const UINT pos,const IMTConRouteDealer* dealer)=0;
   virtual MTAPIRES  DealerDelete(const UINT pos)=0;
   virtual MTAPIRES  DealerClear(void)=0;
   virtual MTAPIRES  DealerShift(const UINT pos,const int shift)=0;
   virtual UINT      DealerTotal(void) const=0;
   virtual MTAPIRES  DealerNext(const UINT pos,IMTConRouteDealer* dealer) const=0;
   virtual MTAPIRES  DealerGet(const UINT64 login,IMTConRouteDealer* dealer) const=0;
   //--- value volume with extended accuracy
   virtual UINT64    ParamVolumeExt(void) const=0;
   virtual MTAPIRES  ParamVolumeExt(const UINT64 value)=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTConRoute(void) {}
  };

class IMTConRouteSink
  {
public:
   virtual void      OnRouteAdd(const IMTConRoute*    /*config*/) {  }
   virtual void      OnRouteUpdate(const IMTConRoute* /*config*/) {  }
   virtual void      OnRouteDelete(const IMTConRoute* /*config*/) {  }
   virtual void      OnRouteSync(void)                            {  }
  };

class IMTUser
  {
public:
   //--- client rights bit flags
   enum EnUsersRights
     {
      USER_RIGHT_NONE             =0x0000000000000000,  // none
      USER_RIGHT_ENABLED          =0x0000000000000001,  // client allowed to connect
      USER_RIGHT_PASSWORD         =0x0000000000000002,  // client allowed to change password
      USER_RIGHT_TRADE_DISABLED   =0x0000000000000004,  // client trading disabled
      USER_RIGHT_INVESTOR         =0x0000000000000008,  // client is investor
      USER_RIGHT_CONFIRMED        =0x0000000000000010,  // client certificate confirmed
      USER_RIGHT_TRAILING         =0x0000000000000020,  // trailing stops are allowed
      USER_RIGHT_EXPERT           =0x0000000000000040,  // expert advisors are allowed
      USER_RIGHT_OBSOLETE         =0x0000000000000080,  // obsolete value
      USER_RIGHT_REPORTS          =0x0000000000000100,  // trade reports are allowed
      USER_RIGHT_READONLY         =0x0000000000000200,  // client is readonly
      USER_RIGHT_RESET_PASS       =0x0000000000000400,  // client must change password at next login
      USER_RIGHT_OTP_ENABLED      =0x0000000000000800,  // client allowed to use one-time password
      USER_RIGHT_SPONSORED_HOSTING=0x0000000000002000,  // client allowed to use sponsored by broker MetaTrader Virtual Hosting
      USER_RIGHT_API_ENABLED      =0x0000000000004000,  // client API are allowed
      USER_RIGHT_PUSH_NOTIFICATION=0x0000000000008000,  // allow to subscribe on trade notifications
      USER_RIGHT_TECHNICAL        =0x0000000000010000,  // technical account
      USER_RIGHT_EXCLUDE_REPORTS  =0x0000000000020000,  // exclude from reports
      //--- enumeration borders
      USER_RIGHT_DEFAULT   =USER_RIGHT_ENABLED |USER_RIGHT_PASSWORD|USER_RIGHT_TRAILING|USER_RIGHT_EXPERT|USER_RIGHT_REPORTS,
      USER_RIGHT_ALL       =USER_RIGHT_ENABLED |USER_RIGHT_PASSWORD |USER_RIGHT_TRADE_DISABLED|
      USER_RIGHT_INVESTOR  | USER_RIGHT_CONFIRMED   | USER_RIGHT_TRAILING |
      USER_RIGHT_EXPERT    | USER_RIGHT_REPORTS     | USER_RIGHT_READONLY |
      USER_RIGHT_RESET_PASS| USER_RIGHT_OTP_ENABLED | USER_RIGHT_SPONSORED_HOSTING | USER_RIGHT_API_ENABLED | USER_RIGHT_PUSH_NOTIFICATION |
      USER_RIGHT_TECHNICAL | USER_RIGHT_EXCLUDE_REPORTS
     };
   //--- password types
   enum EnUsersPasswords
     {
      USER_PASS_MAIN    =0,
      USER_PASS_INVESTOR=1,
      USER_PASS_API     =2,
      //--- enumeration borders
      USER_PASS_FIRST   =USER_PASS_MAIN,
      USER_PASS_LAST    =USER_PASS_API,
     };
   //--- connection types
   enum EnUsersConnectionTypes
     {
      //--- client types
      USER_TYPE_CLIENT            =0,
      USER_TYPE_CLIENT_WINMOBILE  =1,
      USER_TYPE_CLIENT_WINPHONE   =2,
      USER_TYPE_CLIENT_API_WEB    =3,
      USER_TYPE_CLIENT_IPHONE     =4,
      USER_TYPE_CLIENT_ANDROID    =5,
      USER_TYPE_CLIENT_BLACKBERRY =6,
      USER_TYPE_CLIENT_WEB        =11,
      //--- manager types
      USER_TYPE_ADMIN             =32,
      USER_TYPE_MANAGER           =33,
      USER_TYPE_MANAGER_API       =34,
      USER_TYPE_ADMIN_API         =36,
      USER_TYPE_MANAGER_API_WEB   =37,
      //--- enumeration borders
      USER_TYPE_FIRST             =USER_TYPE_CLIENT,
      USER_TYPE_LAST              =USER_TYPE_MANAGER_API_WEB
     };
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTUser* user)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- login
   virtual UINT64    Login(void) const=0;
   virtual MTAPIRES  Login(const UINT64 login)=0;
   //--- group
   virtual LPCWSTR   Group(void) const=0;
   virtual MTAPIRES  Group(LPCWSTR group)=0;
   //--- certificate serial number
   virtual UINT64    CertSerialNumber(void) const=0;
   //--- EnUsersRights
   virtual UINT64    Rights(void) const=0;
   virtual MTAPIRES  Rights(const UINT64 rights)=0;
   //--- registration datetime (filled by MT5)
   virtual INT64     Registration(void) const=0;
   virtual INT64     LastAccess(void) const =0;
   virtual LPCWSTR   LastIP(MTAPISTR& ip) const=0;
   //--- name
   virtual LPCWSTR   Name(void) const=0;
   virtual MTAPIRES  Name(LPCWSTR name)=0;
   //--- company
   virtual LPCWSTR   Company(void) const=0;
   virtual MTAPIRES  Company(LPCWSTR id)=0;
   //--- external system account (exchange, ECN, etc)
   virtual LPCWSTR   Account(void) const=0;
   virtual MTAPIRES  Account(LPCWSTR account)=0;
   //--- country
   virtual LPCWSTR   Country(void) const=0;
   virtual MTAPIRES  Country(LPCWSTR account)=0;
   //--- client language (WinAPI LANGID)
   virtual UINT      Language(void) const=0;
   virtual MTAPIRES  Language(const UINT language)=0;
   //--- city
   virtual LPCWSTR   City(void) const=0;
   virtual MTAPIRES  City(LPCWSTR city)=0;
   //--- state
   virtual LPCWSTR   State(void) const=0;
   virtual MTAPIRES  State(LPCWSTR state)=0;
   //--- ZIP code
   virtual LPCWSTR   ZIPCode(void) const=0;
   virtual MTAPIRES  ZIPCode(LPCWSTR code)=0;
   //--- address
   virtual LPCWSTR   Address(void) const=0;
   virtual MTAPIRES  Address(LPCWSTR code)=0;
   //--- phone
   virtual LPCWSTR   Phone(void) const=0;
   virtual MTAPIRES  Phone(LPCWSTR phone)=0;
   //--- email
   virtual LPCWSTR   EMail(void) const=0;
   virtual MTAPIRES  EMail(LPCWSTR email)=0;
   //--- additional ID
   virtual LPCWSTR   ID(void) const=0;
   virtual MTAPIRES  ID(LPCWSTR email)=0;
   //--- additional status
   virtual LPCWSTR   Status(void) const=0;
   virtual MTAPIRES  Status(LPCWSTR id)=0;
   //--- comment
   virtual LPCWSTR   Comment(void) const=0;
   virtual MTAPIRES  Comment(LPCWSTR comment)=0;
   //--- color
   virtual COLORREF  Color(void) const=0;
   virtual MTAPIRES  Color(const COLORREF color)=0;
   //--- phone password
   virtual LPCWSTR   PhonePassword(void) const=0;
   virtual MTAPIRES  PhonePassword(LPCWSTR password)=0;
   //--- leverage
   virtual UINT      Leverage(void) const=0;
   virtual MTAPIRES  Leverage(const UINT leverage)=0;
   //--- agent account
   virtual UINT64    Agent(void) const=0;
   virtual MTAPIRES  Agent(const UINT64 agent)=0;
   //--- balance & credit
   virtual double    Balance(void) const=0;
   virtual double    Credit(void) const=0;
   //--- accumulated interest rate
   virtual double    InterestRate(void) const=0;
   //--- accumulated daily and monthly commissions
   virtual double    CommissionDaily(void) const=0;
   virtual double    CommissionMonthly(void) const=0;
   //--- accumulated daily and monthly agent commissions
   virtual double    CommissionAgentDaily(void) const=0;
   virtual double    CommissionAgentMonthly(void) const=0;
   //--- previous balance state
   virtual double    BalancePrevDay(void) const=0;
   virtual double    BalancePrevMonth(void) const=0;
   //--- previous equity state
   virtual double    EquityPrevDay(void) const=0;
   virtual double    EquityPrevMonth(void) const=0;
   //--- user record internal data for API usage
   virtual MTAPIRES  ApiDataSet(const USHORT app_id,const UCHAR id,const INT64 value)=0;
   virtual MTAPIRES  ApiDataSet(const USHORT app_id,const UCHAR id,const UINT64 value)=0;
   virtual MTAPIRES  ApiDataSet(const USHORT app_id,const UCHAR id,const double value)=0;
   virtual MTAPIRES  ApiDataGet(const USHORT app_id,const UCHAR id,INT64& value) const=0;
   virtual MTAPIRES  ApiDataGet(const USHORT app_id,const UCHAR id,UINT64& value) const=0;
   virtual MTAPIRES  ApiDataGet(const USHORT app_id,const UCHAR id,double& value) const=0;
   virtual MTAPIRES  ApiDataClear(const USHORT app_id)=0;
   virtual MTAPIRES  ApiDataClearAll(void)=0;
   //--- external accounts
   virtual MTAPIRES  ExternalAccountAdd(const UINT64 gateway_id,LPCWSTR account)=0;
   virtual MTAPIRES  ExternalAccountUpdate(const UINT pos,const UINT64 gateway_id,LPCWSTR account)=0;
   virtual MTAPIRES  ExternalAccountDelete(const UINT pos)=0;
   virtual MTAPIRES  ExternalAccountClear(void)=0;
   virtual UINT      ExternalAccountTotal(void) const=0;
   virtual MTAPIRES  ExternalAccountNext(const UINT pos,UINT64& gateway_id,MTAPISTR& account) const=0;
   virtual MTAPIRES  ExternalAccountGet(const UINT64 gateway_id,MTAPISTR& account) const=0;
   //--- last password change datetime (filled by MT5)
   virtual INT64     LastPassChange(void) const =0;
   //--- client's MetaQuotes ID
   virtual LPCWSTR   MQID(MTAPISTR& mqid) const=0;
   //--- user record internal data for API usage
   virtual MTAPIRES  ApiDataUpdate(const UINT pos,const USHORT app_id,const UCHAR id,const INT64 value)=0;
   virtual MTAPIRES  ApiDataUpdate(const UINT pos,const USHORT app_id,const UCHAR id,const UINT64 value)=0;
   virtual MTAPIRES  ApiDataUpdate(const UINT pos,const USHORT app_id,const UCHAR id,const double value)=0;
   virtual MTAPIRES  ApiDataNext(const UINT pos,USHORT& app_id,UCHAR& id,INT64& value) const=0;
   virtual MTAPIRES  ApiDataNext(const UINT pos,USHORT& app_id,UCHAR& id,UINT64& value) const=0;
   virtual MTAPIRES  ApiDataNext(const UINT pos,USHORT& app_id,UCHAR& id,double& value) const=0;
   //--- clients passwords derivative hash for password comparison
   virtual MTAPIRES  PasswordHash(const UINT type,MTAPISTR& password_hash) const=0;
   //--- lead campaign
   virtual LPCWSTR   LeadCampaign(void) const=0;
   virtual MTAPIRES  LeadCampaign(LPCWSTR lead_campaign)=0;
   //--- lead source
   virtual LPCWSTR   LeadSource(void) const=0;
   virtual MTAPIRES  LeadSource(LPCWSTR lead_source)=0;
   //--- client id
   virtual UINT64    ClientID(void) const=0;
   virtual MTAPIRES  ClientID(const UINT64 id)=0;
   //--- first name
   virtual LPCWSTR   FirstName(void) const=0;
   virtual MTAPIRES  FirstName(LPCWSTR first_name)=0;
   //--- last name
   virtual LPCWSTR   LastName(void) const=0;
   virtual MTAPIRES  LastName(LPCWSTR last_name)=0;
   //--- middle name
   virtual LPCWSTR   MiddleName(void) const=0;
   virtual MTAPIRES  MiddleName(LPCWSTR middle_name)=0;
   //--- registration datetime (filled by MT5)
   virtual MTAPIRES  RegistrationSet(const INT64 datetime)=0;
   //--- otp secret
   virtual LPCWSTR   OTPSecret(void) const=0;
   virtual MTAPIRES  OTPSecret(LPCWSTR otp_secret)=0;
   //--- open orders limit
   virtual UINT      LimitOrders(void) const=0;
   virtual MTAPIRES  LimitOrders(const UINT id)=0;
   //--- open positions value limit
   virtual double    LimitPositionsValue(void) const=0;
   virtual MTAPIRES  LimitPositionsValue(const double value)=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTUser(void) {}
  };

class IMTUserArray
  {
public:
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTUserArray* array)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- add
   virtual MTAPIRES  Add(IMTUser* user)=0;
   virtual MTAPIRES  AddCopy(const IMTUser* user)=0;
   virtual MTAPIRES  Add(IMTUserArray* array)=0;
   virtual MTAPIRES  AddCopy(const IMTUserArray* array)=0;
   //--- delete & detach
   virtual MTAPIRES  Delete(const UINT pos)=0;
   virtual IMTUser*  Detach(const UINT pos)=0;
   //--- update
   virtual MTAPIRES  Update(const UINT pos,IMTUser* account)=0;
   virtual MTAPIRES  UpdateCopy(const UINT pos,const IMTUser* account)=0;
   virtual MTAPIRES  Shift(const UINT pos,const int shift)=0;
   //--- data access
   virtual UINT      Total(void) const=0;
   virtual IMTUser*  Next(const UINT index) const=0;
   //--- sorting and search
   virtual MTAPIRES  Sort(MTSortFunctionPtr sort_function)=0;
   virtual int       Search(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchGreatOrEq(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchGreater(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchLessOrEq(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchLess(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchLeft(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchRight(const void *key,MTSortFunctionPtr sort_function) const=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTUserArray(void) {}
  };

class IMTOnline
  {
public:
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTOnline* online)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- unique session id
   virtual UINT64    SessionID(void) const=0;
   //--- client login
   virtual UINT64    Login(void) const=0;
   //--- client group
   virtual LPCWSTR   Group(void) const=0;
   //--- client ip
   virtual LPCWSTR   Address(MTAPISTR& ip) const=0;
   //--- client terminal type from IMTUser::EnUsersConnectionTypes
   virtual UINT      Type(void) const=0;
   //--- client terminal build
   virtual UINT      Build(void) const=0;
   //--- connection time
   virtual INT64     Time(void) const=0;
   //--- unique computer id
   virtual LPCWSTR   ComputerID(MTAPISTR& cid) const=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTOnline(void) {}
  };

class IMTAccount
  {
public:
   //--- activation mode
   enum EnSoActivation
     {
      ACTIVATION_NONE       =0,
      ACTIVATION_MARGIN_CALL=1,
      ACTIVATION_STOP_OUT   =2,
      //---
      ACTIVATION_FIRST      =ACTIVATION_NONE,
      ACTIVATION_LAST       =ACTIVATION_STOP_OUT,
     };
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTAccount* user)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- login
   virtual UINT64    Login(void) const=0;
   virtual MTAPIRES  Login(const UINT64 login)=0;
   //--- currency digits
   virtual UINT      CurrencyDigits(void) const=0;
   virtual MTAPIRES  CurrencyDigits(const UINT digits)=0;
   //--- balance
   virtual double    Balance(void) const=0;
   virtual MTAPIRES  Balance(const double balance)=0;
   //--- credit
   virtual double    Credit(void) const=0;
   virtual MTAPIRES  Credit(const double credit)=0;
   //--- margin
   virtual double    Margin(void) const=0;
   virtual MTAPIRES  Margin(const double margin)=0;
   //--- free margin
   virtual double    MarginFree(void) const=0;
   virtual MTAPIRES  MarginFree(const double margin_free)=0;
   //--- margin level
   virtual double    MarginLevel(void) const=0;
   virtual MTAPIRES  MarginLevel(const double margin_level)=0;
   //--- margin leverage
   virtual UINT      MarginLeverage(void) const=0;
   virtual MTAPIRES  MarginLeverage(const UINT leverage)=0;
   //--- floating profit
   virtual double    Profit(void) const=0;
   virtual MTAPIRES  Profit(const double profit)=0;
   //--- storage
   virtual double    Storage(void) const=0;
   virtual MTAPIRES  Storage(const double storage)=0;
   //--- obsolete value
   virtual double    ObsoleteValue(void) const=0;
   virtual MTAPIRES  ObsoleteValue(const double value)=0;
   //--- cumulative floating
   virtual double    Floating(void) const=0;
   virtual MTAPIRES  Floating(const double floating)=0;
   //--- equity
   virtual double    Equity(void) const=0;
   virtual MTAPIRES  Equity(const double equity)=0;
   //--- stop-out activation mode
   virtual UINT      SOActivation(void) const=0;
   virtual MTAPIRES  SOActivation(const UINT activation)=0;
   //--- stop-out activation time
   virtual INT64     SOTime(void) const=0;
   virtual MTAPIRES  SOTime(const INT64 datetime)=0;
   //--- margin level on stop-out 
   virtual double    SOLevel(void) const=0;
   virtual MTAPIRES  SOLevel(const double level)=0;
   //--- equity on stop-out
   virtual double    SOEquity(void) const=0;
   virtual MTAPIRES  SOEquity(const double equity)=0;
   //--- margin on stop-out
   virtual double    SOMargin(void) const=0;
   virtual MTAPIRES  SOMargin(const double margin)=0;
   //--- blocked daily & monthly commission
   virtual double    BlockedCommission(void) const=0;
   virtual MTAPIRES  BlockedCommission(const double commission)=0;
   //--- blocked fixed profit
   virtual double    BlockedProfit(void) const=0;
   virtual MTAPIRES  BlockedProfit(const double profit)=0;
   //--- account initial margin
   virtual double    MarginInitial(void) const=0;
   virtual MTAPIRES  MarginInitial(const double margin)=0;
   //--- account maintenance margin 
   virtual double    MarginMaintenance(void) const=0;
   virtual MTAPIRES  MarginMaintenance(const double margin)=0;
   //--- account assets
   virtual double    Assets(void) const=0;
   virtual MTAPIRES  Assets(const double assets)=0;
   //--- account liabilities
   virtual double    Liabilities(void) const=0;
   virtual MTAPIRES  Liabilities(const double liabilities)=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTAccount(void) {}
  };

class IMTUserSink
  {
public:
   //--- events
   virtual void      OnUserAdd(const IMTUser*    /*user*/)           {  }
   virtual void      OnUserUpdate(const IMTUser* /*user*/)           {  }
   virtual void      OnUserDelete(const IMTUser* /*user*/)           {  }
   virtual void      OnUserClean(const UINT64 /*login*/)             {  }
   virtual void      OnUserLogin(LPCWSTR /*ip*/,const IMTUser* /*user*/,const UINT /*type*/) {  }
   virtual void      OnUserSync(void)                                {  }
   //--- hooks
   virtual MTAPIRES  HookUserAdd(IMTUser* /*user*/)                                            { return(MT_RET_OK); }
   virtual MTAPIRES  HookUserUpdate(const IMTUser* /*prev*/,IMTUser* /*user*/)                 { return(MT_RET_OK); }
   virtual MTAPIRES  HookUserDelete(const IMTUser* /*user*/)                                   { return(MT_RET_OK); }
   virtual MTAPIRES  HookUserLogin(LPCWSTR /*ip*/,const IMTUser* /*user*/,const UINT /*type*/) { return(MT_RET_OK); }
   //--- events
   virtual void      OnUserLogout(LPCWSTR /*ip*/,const IMTUser* /*user*/,const UINT /*type*/)  {                    }
   virtual void      OnUserArchive(const IMTUser*  /*user*/)                                   {                    }
   virtual void      OnUserRestore(const IMTUser*  /*user*/)                                   {                    }
   //--- hooks
   virtual MTAPIRES  HookUserArchive(const IMTUser*  /*user*/)                                 { return(MT_RET_OK); }
   virtual MTAPIRES  HookUserLoginExt(const IMTUser* /*user*/,const IMTOnline* /*online*/)     { return(MT_RET_OK); }
   //--- events
   virtual void      OnUserLoginExt(const IMTUser* /*user*/,const IMTOnline* /*online*/)       {  }
   virtual void      OnUserLogoutExt(const IMTUser* /*user*/,const IMTOnline* /*online*/)      {  }
   virtual void      OnUserAddExt(const IMTUser* /*user*/,LPCWSTR /*master_password*/,LPCWSTR /*investor_password*/)   {  }
   virtual void      OnUserChangePassword(const IMTUser* /*user*/,const UINT /*password_type*/,LPCWSTR /*password*/)   {  }
   //--- hooks
   virtual MTAPIRES  HookUserAddExt(IMTUser* /*user*/,LPCWSTR /*master_password*/,LPCWSTR /*investor_password*/) { return(MT_RET_OK); }
   virtual MTAPIRES  HookUserChangePassword(const IMTUser* /*user*/,const UINT /*password_type*/,LPCWSTR /*password*/) { return(MT_RET_OK); }
  };

class IMTPosition
  {
public:
   //--- position types
   enum EnPositionAction
     {
      POSITION_BUY             =0,     // buy
      POSITION_SELL            =1,     // sell
      //--- enumeration borders
      POSITION_FIRST           =POSITION_BUY,
      POSITION_LAST            =POSITION_SELL
     };
   //--- activation modes
   enum EnActivation
     {
      ACTIVATION_NONE          =0,     // none
      ACTIVATION_SL            =1,     // SL activated
      ACTIVATION_TP            =2,     // TP activated
      ACTIVATION_STOPOUT       =3,     // Stop-Out activated
      //--- enumeration borders
      ACTIVATION_FIRST  =ACTIVATION_NONE,
      ACTIVATION_LAST   =ACTIVATION_STOPOUT,
     };
   //--- position activation flags
   enum EnTradeActivationFlags
     {
      ACTIV_FLAGS_NO_LIMIT     =0x00000001,
      ACTIV_FLAGS_NO_STOP      =0x00000002,
      ACTIV_FLAGS_NO_SLIMIT    =0x00000004,
      ACTIV_FLAGS_NO_SL        =0x00000008,
      ACTIV_FLAGS_NO_TP        =0x00000010,
      ACTIV_FLAGS_NO_SO        =0x00000020,
      ACTIV_FLAGS_NO_EXPIRATION=0x00000040,
      //--- 
      ACTIV_FLAGS_NONE         =0x00000000,
      ACTIV_FLAGS_ALL          =ACTIV_FLAGS_NO_LIMIT|ACTIV_FLAGS_NO_STOP|ACTIV_FLAGS_NO_SLIMIT|ACTIV_FLAGS_NO_SL|
      ACTIV_FLAGS_NO_TP|ACTIV_FLAGS_NO_SO|ACTIV_FLAGS_NO_EXPIRATION
     };
   //--- position creation reasons
   enum EnPositionReason
     {
      POSITION_REASON_CLIENT   =0,     // position placed manually
      POSITION_REASON_EXPERT   =1,     // position placed by expert
      POSITION_REASON_DEALER   =2,     // position placed by dealer
      POSITION_REASON_SL       =3,     // position placed due SL
      POSITION_REASON_TP       =4,     // position placed due TP
      POSITION_REASON_SO       =5,     // position placed due Stop-Out
      POSITION_REASON_ROLLOVER =6,     // position placed due rollover
      POSITION_REASON_EXTERNAL_CLIENT=7,  // position placed from the external system by client
      POSITION_REASON_VMARGIN  =8,     // position placed due variation margin
      POSITION_REASON_GATEWAY  =9,     // position placed by gateway
      POSITION_REASON_SIGNAL   =10,    // position placed by signal service
      POSITION_REASON_SETTLEMENT=11,   // position placed due settlement
      POSITION_REASON_TRANSFER =12,    // position placed due position transfer
      POSITION_REASON_SYNC     =13,    // position placed due position synchronization
      POSITION_REASON_EXTERNAL_SERVICE=14, // position placed from the external system due service issues
      POSITION_REASON_MIGRATION=15,    // position placed due migration
      POSITION_REASON_MOBILE   =16,    // position placed by mobile terminal
      POSITION_REASON_WEB      =17,    // position placed by web terminal
      POSITION_REASON_SPLIT    =18,    // position placed due split
      //--- enumeration borders
      POSITION_REASON_FIRST    =POSITION_REASON_CLIENT,
      POSITION_REASON_LAST     =POSITION_REASON_SPLIT
     };
   //--- modification flags
   enum EnTradeModifyFlags
     {
      MODIFY_FLAGS_ADMIN       =0x00000001,
      MODIFY_FLAGS_MANAGER     =0x00000002,
      MODIFY_FLAGS_POSITION    =0x00000004,
      MODIFY_FLAGS_RESTORE     =0x00000008,
      MODIFY_FLAGS_API_ADMIN   =0x00000010,
      MODIFY_FLAGS_API_MANAGER =0x00000020,
      MODIFY_FLAGS_API_SERVER  =0x00000040,
      MODIFY_FLAGS_API_GATEWAY =0x00000080,
      //--- enumeration borders
      MODIFY_FLAGS_NONE         =0x00000000,
      MODIFY_FLAGS_ALL          =MODIFY_FLAGS_ADMIN|MODIFY_FLAGS_MANAGER|MODIFY_FLAGS_POSITION|MODIFY_FLAGS_RESTORE|
      MODIFY_FLAGS_API_ADMIN|MODIFY_FLAGS_API_MANAGER|MODIFY_FLAGS_API_SERVER|MODIFY_FLAGS_API_GATEWAY
     };
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTPosition* position)=0;
   virtual MTAPIRES  Clear(void)=0;
   virtual LPCWSTR   Print(MTAPISTR& string) const=0;
   //--- owner client login
   virtual UINT64    Login(void) const=0;
   //--- position symbol
   virtual LPCWSTR   Symbol(void) const=0;
   virtual MTAPIRES  Symbol(LPCWSTR symbol)=0;
   //--- EnPositionAction
   virtual UINT      Action(void) const=0;
   virtual MTAPIRES  Action(const UINT action)=0;
   //--- price digits
   virtual UINT      Digits(void) const=0;
   virtual MTAPIRES  Digits(const UINT digits)=0;
   //--- currency digits
   virtual UINT      DigitsCurrency(void) const=0;
   virtual MTAPIRES  DigitsCurrency(const UINT digits)=0;
   //--- symbol contract size
   virtual double    ContractSize(void) const=0;
   virtual MTAPIRES  ContractSize(const double contract_size)=0;
   //--- position create time
   virtual INT64     TimeCreate(void) const=0;
   virtual MTAPIRES  TimeCreate(const INT64 time)=0;
   //--- position last update time
   virtual INT64     TimeUpdate(void) const=0;
   virtual MTAPIRES  TimeUpdate(const INT64 time)=0;
   //--- position weighted average open price
   virtual double    PriceOpen(void) const=0;
   virtual MTAPIRES  PriceOpen(const double price)=0;
   //--- position current price
   virtual double    PriceCurrent(void) const=0;
   virtual MTAPIRES  PriceCurrent(const double price)=0;
   //--- position SL price
   virtual double    PriceSL(void) const=0;
   virtual MTAPIRES  PriceSL(const double price)=0;
   //--- position TP price
   virtual double    PriceTP(void) const=0;
   virtual MTAPIRES  PriceTP(const double price)=0;
   //--- position volume
   virtual UINT64    Volume(void) const=0;
   virtual MTAPIRES  Volume(const UINT64 volume)=0;
   //--- position floating profit
   virtual double    Profit(void) const=0;
   virtual MTAPIRES  Profit(const double profit)=0;
   //--- position accumulated swaps
   virtual double    Storage(void) const=0;
   virtual MTAPIRES  Storage(const double storage)=0;
   //--- obsolete value
   virtual double    ObsoleteValue(void) const=0;
   virtual MTAPIRES  ObsoleteValue(const double value)=0;
   //--- profit conversion rate (from symbol profit currency to deposit currency)
   virtual double    RateProfit(void) const=0;
   virtual MTAPIRES  RateProfit(const double rate)=0;
   //--- margin conversion rate (from symbol margin currency to deposit currency)
   virtual double    RateMargin(void) const=0;
   virtual MTAPIRES  RateMargin(const double rate)=0;
   //--- expert id (filled by expert advisor)
   virtual UINT64    ExpertID(void) const=0;
   virtual MTAPIRES  ExpertID(const UINT64 id)=0;
   //--- expert position id
   virtual UINT64    ExpertPositionID(void) const=0;
   virtual MTAPIRES  ExpertPositionID(const UINT64 id)=0;
   //--- comment
   virtual LPCWSTR   Comment(void) const=0;
   virtual MTAPIRES  Comment(LPCWSTR comment)=0;
   //--- order activation state, time and price
   virtual UINT      ActivationMode(void) const=0;
   virtual INT64     ActivationTime(void) const=0;
   virtual double    ActivationPrice(void) const=0;
   virtual UINT      ActivationFlags(void) const=0;
   //--- position internal data for API usage
   virtual MTAPIRES  ApiDataSet(const USHORT app_id,const UCHAR id,const INT64 value)=0;
   virtual MTAPIRES  ApiDataSet(const USHORT app_id,const UCHAR id,const UINT64 value)=0;
   virtual MTAPIRES  ApiDataSet(const USHORT app_id,const UCHAR id,const double value)=0;
   virtual MTAPIRES  ApiDataGet(const USHORT app_id,const UCHAR id,INT64& value) const=0;
   virtual MTAPIRES  ApiDataGet(const USHORT app_id,const UCHAR id,UINT64& value) const=0;
   virtual MTAPIRES  ApiDataGet(const USHORT app_id,const UCHAR id,double& value) const=0;
   virtual MTAPIRES  ApiDataClear(const USHORT app_id)=0;
   virtual MTAPIRES  ApiDataClearAll(void)=0;
   //--- position create time in msc since 1970.01.01
   virtual INT64     TimeCreateMsc(void) const=0;
   virtual MTAPIRES  TimeCreateMsc(const INT64 time)=0;
   //--- position last update time in msc since 1970.01.01
   virtual INT64     TimeUpdateMsc(void) const=0;
   virtual MTAPIRES  TimeUpdateMsc(const INT64 time)=0;
   //--- order activation state, time and price
   virtual MTAPIRES  ActivationMode(const UINT mode)=0;
   virtual MTAPIRES  ActivationTime(const INT64 atm)=0;
   virtual MTAPIRES  ActivationPrice(const double price)=0;
   virtual MTAPIRES  ActivationFlags(const UINT flags)=0;
   //--- processed dealer login (0-means auto) (first position deal dealer)
   virtual UINT64    Dealer(void) const=0;
   virtual MTAPIRES  Dealer(const UINT64 dealer)=0;
   //--- user record internal data for API usage
   virtual MTAPIRES  ApiDataUpdate(const UINT pos,const USHORT app_id,const UCHAR id,const INT64 value)=0;
   virtual MTAPIRES  ApiDataUpdate(const UINT pos,const USHORT app_id,const UCHAR id,const UINT64 value)=0;
   virtual MTAPIRES  ApiDataUpdate(const UINT pos,const USHORT app_id,const UCHAR id,const double value)=0;
   virtual MTAPIRES  ApiDataNext(const UINT pos,USHORT& app_id,UCHAR& id,INT64& value) const=0;
   virtual MTAPIRES  ApiDataNext(const UINT pos,USHORT& app_id,UCHAR& id,UINT64& value) const=0;
   virtual MTAPIRES  ApiDataNext(const UINT pos,USHORT& app_id,UCHAR& id,double& value) const=0;
   //--- owner client login
   virtual MTAPIRES  LoginSet(const UINT64 login)=0;
   //--- position ticket
   virtual UINT64    Position(void) const=0;
   //--- position ticket in external system (exchange, ECN, etc)
   virtual LPCWSTR   ExternalID(void) const=0;
   virtual MTAPIRES  ExternalID(LPCWSTR id)=0;
   //--- modification flags
   virtual UINT      ModificationFlags(void) const=0;
   //--- position reason - EnPositionReason
   virtual UINT      Reason(void) const=0;
   //--- position volume
   virtual UINT64    VolumeExt(void) const=0;
   virtual MTAPIRES  VolumeExt(const UINT64 volume)=0;
   //--- EnPositionReason
   virtual MTAPIRES  ReasonSet(const UINT reason)=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTPosition(void) {}
  };

class IMTPositionArray
  {
public:
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTPositionArray* array)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- add
   virtual MTAPIRES  Add(IMTPosition* position)=0;
   virtual MTAPIRES  AddCopy(const IMTPosition* position)=0;
   virtual MTAPIRES  Add(IMTPositionArray* array)=0;
   virtual MTAPIRES  AddCopy(const IMTPositionArray* array)=0;
   //--- delete & detach
   virtual MTAPIRES  Delete(const UINT pos)=0;
   virtual IMTPosition* Detach(const UINT pos)=0;
   //--- update
   virtual MTAPIRES  Update(const UINT pos,IMTPosition* position)=0;
   virtual MTAPIRES  UpdateCopy(const UINT pos,const IMTPosition* position)=0;
   virtual MTAPIRES  Shift(const UINT pos,const int shift)=0;
   //--- data access
   virtual UINT      Total(void) const=0;
   virtual IMTPosition* Next(const UINT index) const=0;
   //--- sorting and search
   virtual MTAPIRES  Sort(MTSortFunctionPtr sort_function)=0;
   virtual int       Search(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchGreatOrEq(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchGreater(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchLessOrEq(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchLess(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchLeft(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchRight(const void *key,MTSortFunctionPtr sort_function) const=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTPositionArray(void) {}
  };

class IMTPositionSink
  {
public:
   virtual void      OnPositionAdd(const IMTPosition* /*position*/)    {  }
   virtual void      OnPositionUpdate(const IMTPosition* /*position*/) {  }
   virtual void      OnPositionDelete(const IMTPosition* /*position*/) {  }
   virtual void      OnPositionClean(const UINT64 /*login*/)           {  }
   virtual void      OnPositionSync(void)                              {  }
  };

class IMTDeal
  {
public:
   //--- deal types
   enum EnDealAction
     {
      DEAL_BUY                 =0,     // buy
      DEAL_SELL                =1,     // sell
      DEAL_BALANCE             =2,     // deposit operation
      DEAL_CREDIT              =3,     // credit operation
      DEAL_CHARGE              =4,     // additional charges
      DEAL_CORRECTION          =5,     // correction deals
      DEAL_BONUS               =6,     // bonus
      DEAL_COMMISSION          =7,     // commission
      DEAL_COMMISSION_DAILY    =8,     // daily commission
      DEAL_COMMISSION_MONTHLY  =9,     // monthly commission
      DEAL_AGENT_DAILY         =10,    // daily agent commission
      DEAL_AGENT_MONTHLY       =11,    // monthly agent commission
      DEAL_INTERESTRATE        =12,    // interest rate charges
      DEAL_BUY_CANCELED        =13,    // canceled buy deal
      DEAL_SELL_CANCELED       =14,    // canceled sell deal
      DEAL_DIVIDEND            =15,    // dividend
      DEAL_DIVIDEND_FRANKED    =16,    // franked dividend
      DEAL_TAX                 =17,    // taxes
      DEAL_AGENT               =18,    // instant agent commission
      DEAL_SO_COMPENSATION     =19,    // negative balance compensation after stop-out
      DEAL_SO_COMPENSATION_CREDIT=20,  // credit compensation after stop-out
      //--- enumeration borders
      DEAL_FIRST               =DEAL_BUY,
      DEAL_LAST                =DEAL_SO_COMPENSATION_CREDIT
     };
   //--- deal entry direction
   enum EnDealEntry
     {
      ENTRY_IN                 =0,     // in market
      ENTRY_OUT                =1,     // out of market
      ENTRY_INOUT              =2,     // reverse
      ENTRY_OUT_BY             =3,     // closed by  hedged position
      //--- enumeration borders
      ENTRY_FIRST              =ENTRY_IN,
      ENTRY_LAST               =ENTRY_OUT_BY
     };
   //--- deal creation reasons
   enum EnDealReason
     {
      DEAL_REASON_CLIENT       =0,     // deal placed manually
      DEAL_REASON_EXPERT       =1,     // deal placed by expert
      DEAL_REASON_DEALER       =2,     // deal placed by dealer
      DEAL_REASON_SL           =3,     // deal placed due SL
      DEAL_REASON_TP           =4,     // deal placed due TP
      DEAL_REASON_SO           =5,     // deal placed due Stop-Out
      DEAL_REASON_ROLLOVER     =6,     // deal placed due rollover
      DEAL_REASON_EXTERNAL_CLIENT=7,   // deal placed from the external system by client
      DEAL_REASON_VMARGIN      =8,     // deal placed due variation margin
      DEAL_REASON_GATEWAY      =9,     // deal placed by gateway
      DEAL_REASON_SIGNAL       =10,    // deal placed by signal service
      DEAL_REASON_SETTLEMENT   =11,    // deal placed due settlement
      DEAL_REASON_TRANSFER     =12,    // deal placed due position transfer
      DEAL_REASON_SYNC         =13,    // deal placed due position synchronization
      DEAL_REASON_EXTERNAL_SERVICE=14, // deal placed from the external system due service issues
      DEAL_REASON_MIGRATION    =15,    // deal placed due migration
      DEAL_REASON_MOBILE       =16,    // deal placed manually by mobile terminal
      DEAL_REASON_WEB          =17,    // deal placed manually by web terminal
      DEAL_REASON_SPLIT        =18,    // deal placed due split
      //--- enumeration borders
      DEAL_REASON_FIRST        =DEAL_REASON_CLIENT,
      DEAL_REASON_LAST         =DEAL_REASON_SPLIT
     };
   //--- modification flags
   enum EnTradeModifyFlags
     {
      MODIFY_FLAGS_ADMIN       =0x00000001,
      MODIFY_FLAGS_MANAGER     =0x00000002,
      MODIFY_FLAGS_POSITION    =0x00000004,
      MODIFY_FLAGS_RESTORE     =0x00000008,
      MODIFY_FLAGS_API_ADMIN   =0x00000010,
      MODIFY_FLAGS_API_MANAGER =0x00000020,
      MODIFY_FLAGS_API_SERVER  =0x00000040,
      MODIFY_FLAGS_API_GATEWAY =0x00000080,
      //--- enumeration borders
      MODIFY_FLAGS_NONE         =0x00000000,
      MODIFY_FLAGS_ALL          =MODIFY_FLAGS_ADMIN|MODIFY_FLAGS_MANAGER|MODIFY_FLAGS_POSITION|MODIFY_FLAGS_RESTORE|
      MODIFY_FLAGS_API_ADMIN|MODIFY_FLAGS_API_MANAGER|MODIFY_FLAGS_API_SERVER|MODIFY_FLAGS_API_GATEWAY
     };
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTDeal* deal)=0;
   virtual MTAPIRES  Clear(void)=0;
   virtual LPCWSTR   Print(MTAPISTR& string) const=0;
   //--- deal ticket
   virtual UINT64    Deal(void) const=0;
   //--- deal ticket in external system (exchange, ECN, etc)
   virtual LPCWSTR   ExternalID(void) const=0;
   virtual MTAPIRES  ExternalID(LPCWSTR id)=0;
   //--- client login
   virtual UINT64    Login(void) const=0;
   virtual MTAPIRES  Login(const UINT64 login)=0;
   //--- processed dealer login (0-means auto)
   virtual UINT64    Dealer(void) const=0;
   virtual MTAPIRES  Dealer(const UINT64 dealer)=0;
   //--- deal order ticket
   virtual UINT64    Order(void) const=0;
   virtual MTAPIRES  Order(const UINT64 order)=0;
   //--- EnDealAction
   virtual UINT      Action(void) const=0;
   virtual MTAPIRES  Action(const UINT action)=0;
   //--- EnEntryFlags
   virtual UINT      Entry(void) const=0;
   virtual MTAPIRES  Entry(const UINT entry)=0;
   //--- price digits
   virtual UINT      Digits(void) const=0;
   virtual MTAPIRES  Digits(const UINT digits)=0;
   //--- currency digits
   virtual UINT      DigitsCurrency(void) const=0;
   virtual MTAPIRES  DigitsCurrency(const UINT digits)=0;
   //--- symbol contract size
   virtual double    ContractSize(void) const=0;
   virtual MTAPIRES  ContractSize(const double contract_size)=0;
   //--- deal creation datetime in seconds
   virtual INT64     Time(void) const=0;
   virtual MTAPIRES  Time(const INT64 time)=0;
   //--- deal symbol
   virtual LPCWSTR   Symbol(void) const=0;
   virtual MTAPIRES  Symbol(LPCWSTR symbol)=0;
   //--- deal price
   virtual double    Price(void) const=0;
   virtual MTAPIRES  Price(const double price)=0;
   //--- deal volume
   virtual UINT64    Volume(void) const=0;
   virtual MTAPIRES  Volume(const UINT64 volume)=0;
   //--- deal profit
   virtual double    Profit(void) const=0;
   virtual MTAPIRES  Profit(const double profit)=0;
   //--- deal collected swaps
   virtual double    Storage(void) const=0;
   virtual MTAPIRES  Storage(const double storage)=0;
   //--- deal commission
   virtual double    Commission(void) const=0;
   virtual MTAPIRES  Commission(const double comm)=0;
   //--- obsolete value
   virtual double    ObsoleteValue(void) const=0;
   virtual MTAPIRES  ObsoleteValue(const double agent)=0;
   //--- profit conversion rate (from symbol profit currency to deposit currency)
   virtual double    RateProfit(void) const=0;
   virtual MTAPIRES  RateProfit(const double rate)=0;
   //--- margin conversion rate (from symbol margin currency to deposit currency)
   virtual double    RateMargin(void) const=0;
   virtual MTAPIRES  RateMargin(const double rate)=0;
   //--- expert id (filled by expert advisor)
   virtual UINT64    ExpertID(void) const=0;
   virtual MTAPIRES  ExpertID(const UINT64 id)=0;
   //--- position id
   virtual UINT64    PositionID(void) const=0;
   virtual MTAPIRES  PositionID(const UINT64 id)=0;
   //--- deal comment
   virtual LPCWSTR   Comment(void) const=0;
   virtual MTAPIRES  Comment(LPCWSTR comment)=0;
   //--- deal internal data for API usage
   virtual MTAPIRES  ApiDataSet(const USHORT app_id,const UCHAR id,const INT64 value)=0;
   virtual MTAPIRES  ApiDataSet(const USHORT app_id,const UCHAR id,const UINT64 value)=0;
   virtual MTAPIRES  ApiDataSet(const USHORT app_id,const UCHAR id,const double value)=0;
   virtual MTAPIRES  ApiDataGet(const USHORT app_id,const UCHAR id,INT64& value) const=0;
   virtual MTAPIRES  ApiDataGet(const USHORT app_id,const UCHAR id,UINT64& value) const=0;
   virtual MTAPIRES  ApiDataGet(const USHORT app_id,const UCHAR id,double& value) const=0;
   virtual MTAPIRES  ApiDataClear(const USHORT app_id)=0;
   virtual MTAPIRES  ApiDataClearAll(void)=0;
   //--- deal profit in symbol's profit currency
   virtual double    ProfitRaw(void) const=0;
   virtual MTAPIRES  ProfitRaw(const double profit)=0;
   //--- closed position  price
   virtual double    PricePosition(void) const=0;
   virtual MTAPIRES  PricePosition(const double price)=0;
   //--- closed volume
   virtual UINT64    VolumeClosed(void) const=0;
   virtual MTAPIRES  VolumeClosed(const UINT64 volume)=0;
   //--- tick value
   virtual double    TickValue(void) const=0;
   virtual MTAPIRES  TickValue(const double value)=0;
   //--- tick size
   virtual double    TickSize(void) const=0;
   virtual MTAPIRES  TickSize(const double size)=0;
   //--- flags
   virtual UINT64    Flags(void) const=0;
   virtual MTAPIRES  Flags(const UINT64 flags)=0;
   //--- deal creation datetime in msc since 1970.01.01
   virtual INT64     TimeMsc(void) const=0;
   virtual MTAPIRES  TimeMsc(const INT64 time)=0;
   //--- EnDealReason
   virtual UINT      Reason(void) const=0;
   //--- source gateway name
   virtual LPCWSTR   Gateway(void) const=0;
   //--- deal price on gateway
   virtual double    PriceGateway(void) const=0;
   //--- user record internal data for API usage
   virtual MTAPIRES  ApiDataUpdate(const UINT pos,const USHORT app_id,const UCHAR id,const INT64 value)=0;
   virtual MTAPIRES  ApiDataUpdate(const UINT pos,const USHORT app_id,const UCHAR id,const UINT64 value)=0;
   virtual MTAPIRES  ApiDataUpdate(const UINT pos,const USHORT app_id,const UCHAR id,const double value)=0;
   virtual MTAPIRES  ApiDataNext(const UINT pos,USHORT& app_id,UCHAR& id,INT64& value) const=0;
   virtual MTAPIRES  ApiDataNext(const UINT pos,USHORT& app_id,UCHAR& id,UINT64& value) const=0;
   virtual MTAPIRES  ApiDataNext(const UINT pos,USHORT& app_id,UCHAR& id,double& value) const=0;
   //--- deal ticket
   virtual MTAPIRES  DealSet(const UINT64 deal)=0;
   //--- modification flags
   virtual UINT      ModificationFlags(void) const=0;
   //--- EnOrderReason
   virtual MTAPIRES  ReasonSet(const UINT reason)=0;
   //--- source gateway name
   virtual MTAPIRES  GatewaySet(LPCWSTR gateway)=0;
   //--- deal price on gateway
   virtual MTAPIRES  PriceGatewaySet(double price_gateway)=0;
   //--- order SL
   virtual double    PriceSL(void) const=0;
   virtual MTAPIRES  PriceSL(const double price)=0;
   //--- order TP
   virtual double    PriceTP(void) const=0;
   virtual MTAPIRES  PriceTP(const double price)=0;
   //--- deal volume with extended accuracy
   virtual UINT64    VolumeExt(void) const=0;
   virtual MTAPIRES  VolumeExt(const UINT64 volume)=0;
   //--- closed volume with extended accuracy
   virtual UINT64    VolumeClosedExt(void) const=0;
   virtual MTAPIRES  VolumeClosedExt(const UINT64 volume)=0;
   //--- fee
   virtual double    Fee(void) const=0;
   virtual MTAPIRES  Fee(const double fee)=0;
   //--- value
   virtual double    Value(void) const=0;
   virtual MTAPIRES  Value(const double value)=0;
   //--- market prices at time of the deal
   virtual double    MarketBid(void) const=0;
   virtual double    MarketAsk(void) const=0;
   virtual double    MarketLast(void) const=0;
   //--- market prices at time of the deal
   virtual MTAPIRES  MarketBidSet(const double price)=0;
   virtual MTAPIRES  MarketAskSet(const double price)=0;
   virtual MTAPIRES  MarketLastSet(const double price)=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTDeal(void) {}
  };

class IMTDealArray
  {
public:
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTDealArray* array)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- add
   virtual MTAPIRES  Add(IMTDeal* deal)=0;
   virtual MTAPIRES  AddCopy(const IMTDeal* deal)=0;
   virtual MTAPIRES  Add(IMTDealArray* array)=0;
   virtual MTAPIRES  AddCopy(const IMTDealArray* array)=0;
   //--- delete & detach
   virtual MTAPIRES  Delete(const UINT pos)=0;
   virtual IMTDeal*  Detach(const UINT pos)=0;
   //--- update
   virtual MTAPIRES  Update(const UINT pos,IMTDeal* deal)=0;
   virtual MTAPIRES  UpdateCopy(const UINT pos,const IMTDeal* deal)=0;
   virtual MTAPIRES  Shift(const UINT pos,const int shift)=0;
   //--- data access
   virtual UINT      Total(void) const=0;
   virtual IMTDeal*  Next(const UINT index) const=0;
   //--- sorting and search
   virtual MTAPIRES  Sort(MTSortFunctionPtr sort_function)=0;
   virtual int       Search(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchGreatOrEq(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchGreater(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchLessOrEq(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchLess(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchLeft(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchRight(const void *key,MTSortFunctionPtr sort_function) const=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTDealArray(void) {}
  };

class IMTDealSink
  {
public:
   virtual void      OnDealAdd(const IMTDeal*    /*deal*/) {  }
   virtual void      OnDealUpdate(const IMTDeal* /*deal*/) {  }
   virtual void      OnDealDelete(const IMTDeal* /*deal*/) {  }
   virtual void      OnDealClean(const UINT64 /*login*/)   {  }
   virtual void      OnDealSync(void)                      {  }
   virtual void      OnDealPerform(const IMTDeal* /*deal*/,IMTAccount* /*account*/,IMTPosition* /*position*/) {  }
   virtual void      OnDealPerformCloseBy(const IMTDeal* /*deal*/,const IMTDeal* /*deal_by*/,IMTAccount* /*account*/,IMTPosition* /*position*/) {  }
  };

class IMTOrder
  {
public:
   //--- order types
   enum EnOrderType
     {
      OP_BUY                   =0,     // buy order
      OP_SELL                  =1,     // sell order
      OP_BUY_LIMIT             =2,     // buy limit order
      OP_SELL_LIMIT            =3,     // sell limit order
      OP_BUY_STOP              =4,     // buy stop order
      OP_SELL_STOP             =5,     // sell stop order
      OP_BUY_STOP_LIMIT        =6,     // buy stop limit order
      OP_SELL_STOP_LIMIT       =7,     // sell stop limit order
      OP_CLOSE_BY              =8,     // close by
      //--- enumeration borders
      OP_FIRST                 =OP_BUY,
      OP_LAST                  =OP_CLOSE_BY
     };
   //--- order filling types
   enum EnOrderFilling
     {
      ORDER_FILL_FOK           =0,     // fill or kill
      ORDER_FILL_IOC           =1,     // immediate or cancel
      ORDER_FILL_RETURN        =2,     // return order in queue
      //--- enumeration borders
      ORDER_FILL_FIRST         =ORDER_FILL_FOK,
      ORDER_FILL_LAST          =ORDER_FILL_RETURN
     };
   //--- order expiration types
   enum EnOrderTime
     {
      ORDER_TIME_GTC           =0,     // good till cancel
      ORDER_TIME_DAY           =1,     // good till day
      ORDER_TIME_SPECIFIED     =2,     // good till specified
      ORDER_TIME_SPECIFIED_DAY =3,     // good till specified day
      //--- enumeration borders
      ORDER_TIME_FIRST         =ORDER_TIME_GTC,
      ORDER_TIME_LAST          =ORDER_TIME_SPECIFIED_DAY
     };
   //--- order state
   enum EnOrderState
     {
      ORDER_STATE_STARTED      =0,     // order started
      ORDER_STATE_PLACED       =1,     // order placed in system
      ORDER_STATE_CANCELED     =2,     // order canceled by client
      ORDER_STATE_PARTIAL      =3,     // order partially filled
      ORDER_STATE_FILLED       =4,     // order filled
      ORDER_STATE_REJECTED     =5,     // order rejected
      ORDER_STATE_EXPIRED      =6,     // order expired
      ORDER_STATE_REQUEST_ADD  =7,     // order requested to add
      ORDER_STATE_REQUEST_MODIFY=8,    // order requested to modify
      ORDER_STATE_REQUEST_CANCEL=9,    // order requested to cancel
      //--- enumeration borders
      ORDER_STATE_FIRST        =ORDER_STATE_STARTED,
      ORDER_STATE_LAST         =ORDER_STATE_REQUEST_CANCEL
     };
   //--- order activation state
   enum EnOrderActivation
     {
      ACTIVATION_NONE          =0,     // none
      ACTIVATION_PENDING       =1,     // pending order activated
      ACTIVATION_STOPLIMIT     =2,     // stop-limit order activated
      ACTIVATION_EXPIRATION    =3,     // order expired
      ACTIVATION_STOPOUT       =4,     // order activate for stop-out
      //--- enumeration borders
      ACTIVATION_FIRST         =ACTIVATION_NONE,
      ACTIVATION_LAST          =ACTIVATION_STOPOUT
     };
   //--- order creation reasons
   enum EnOrderReason
     {
      ORDER_REASON_CLIENT      =0,     // order placed manually
      ORDER_REASON_EXPERT      =1,     // order placed by expert
      ORDER_REASON_DEALER      =2,     // order placed by dealer
      ORDER_REASON_SL          =3,     // order placed due SL
      ORDER_REASON_TP          =4,     // order placed due TP
      ORDER_REASON_SO          =5,     // order placed due Stop-Out
      ORDER_REASON_ROLLOVER    =6,     // order placed due rollover
      ORDER_REASON_EXTERNAL_CLIENT =7, // order placed from the external system by client
      ORDER_REASON_VMARGIN     =8,     // order placed due variation margin
      ORDER_REASON_GATEWAY     =9,     // order placed by gateway
      ORDER_REASON_SIGNAL      =10,    // order placed by signal service
      ORDER_REASON_SETTLEMENT  =11,    // order placed by settlement
      ORDER_REASON_TRANSFER    =12,    // order placed due transfer
      ORDER_REASON_SYNC        =13,    // order placed due synchronization
      ORDER_REASON_EXTERNAL_SERVICE=14,// order placed from the external system due service issues
      ORDER_REASON_MIGRATION   =15,    // order placed due account migration from MetaTrader 4 or MetaTrader 5
      ORDER_REASON_MOBILE      =16,    // order placed manually by mobile terminal
      ORDER_REASON_WEB         =17,    // order placed manually by web terminal
      ORDER_REASON_SPLIT       =18,    // order placed due split
      //--- enumeration borders
      ORDER_REASON_FIRST        =ORDER_REASON_CLIENT,
      ORDER_REASON_LAST         =ORDER_REASON_SPLIT
     };
   //--- order activation flags
   enum EnTradeActivationFlags
     {
      ACTIV_FLAGS_NO_LIMIT     =0x00000001,
      ACTIV_FLAGS_NO_STOP      =0x00000002,
      ACTIV_FLAGS_NO_SLIMIT    =0x00000004,
      ACTIV_FLAGS_NO_SL        =0x00000008,
      ACTIV_FLAGS_NO_TP        =0x00000010,
      ACTIV_FLAGS_NO_SO        =0x00000020,
      ACTIV_FLAGS_NO_EXPIRATION=0x00000040,
      //--- enumeration borders
      ACTIV_FLAGS_NONE         =0x00000000,
      ACTIV_FLAGS_ALL          =ACTIV_FLAGS_NO_LIMIT|ACTIV_FLAGS_NO_STOP|ACTIV_FLAGS_NO_SLIMIT|ACTIV_FLAGS_NO_SL|
      ACTIV_FLAGS_NO_TP|ACTIV_FLAGS_NO_SO|ACTIV_FLAGS_NO_EXPIRATION
     };
   //--- modification flags
   enum EnTradeModifyFlags
     {
      MODIFY_FLAGS_ADMIN       =0x00000001,
      MODIFY_FLAGS_MANAGER     =0x00000002,
      MODIFY_FLAGS_POSITION    =0x00000004,
      MODIFY_FLAGS_RESTORE     =0x00000008,
      MODIFY_FLAGS_API_ADMIN   =0x00000010,
      MODIFY_FLAGS_API_MANAGER =0x00000020,
      MODIFY_FLAGS_API_SERVER  =0x00000040,
      MODIFY_FLAGS_API_GATEWAY =0x00000080,
      //--- enumeration borders
      MODIFY_FLAGS_NONE        =0x00000000,
      MODIFY_FLAGS_ALL         =MODIFY_FLAGS_ADMIN|MODIFY_FLAGS_MANAGER|MODIFY_FLAGS_POSITION|MODIFY_FLAGS_RESTORE|
      MODIFY_FLAGS_API_ADMIN|MODIFY_FLAGS_API_MANAGER|MODIFY_FLAGS_API_SERVER|MODIFY_FLAGS_API_GATEWAY
     };
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTOrder* order)=0;
   virtual MTAPIRES  Clear(void)=0;
   virtual LPCWSTR   Print(MTAPISTR& string) const=0;
   //--- order ticket
   virtual UINT64    Order(void) const=0;
   //--- order ticket in external system (exchange, ECN, etc)
   virtual LPCWSTR   ExternalID(void) const=0;
   virtual MTAPIRES  ExternalID(LPCWSTR id)=0;
   //--- client login
   virtual UINT64    Login(void) const=0;
   virtual MTAPIRES  Login(const UINT64 order)=0;
   //--- processed dealer login (0-means auto)
   virtual UINT64    Dealer(void) const=0;
   virtual MTAPIRES  Dealer(const UINT64 dealer)=0;
   //--- order symbol
   virtual LPCWSTR   Symbol(void) const=0;
   virtual MTAPIRES  Symbol(LPCWSTR symbol)=0;
   //--- price digits
   virtual UINT      Digits(void) const=0;
   virtual MTAPIRES  Digits(const UINT digits)=0;
   //--- currency digits
   virtual UINT      DigitsCurrency(void) const=0;
   virtual MTAPIRES  DigitsCurrency(const UINT digits)=0;
   //--- contract size
   virtual double    ContractSize(void) const=0;
   virtual MTAPIRES  ContractSize(const double contract_size)=0;
   //--- EnOrderState
   virtual UINT      State(void) const=0;
   //--- EnOrderReason
   virtual UINT      Reason(void) const=0;
   //--- order setup time
   virtual INT64     TimeSetup(void) const=0;
   virtual MTAPIRES  TimeSetup(const INT64 time)=0;
   //--- order expiration
   virtual INT64     TimeExpiration(void) const=0;
   virtual MTAPIRES  TimeExpiration(const INT64 time)=0;
   //--- order filling/cancel time
   virtual INT64     TimeDone(void) const=0;
   virtual MTAPIRES  TimeDone(const INT64 time)=0;
   //--- EnOrderType
   virtual UINT      Type(void) const=0;
   virtual MTAPIRES  Type(const UINT type)=0;
   //--- EnOrderFilling
   virtual UINT      TypeFill(void) const=0;
   virtual MTAPIRES  TypeFill(const UINT type)=0;
   //--- EnOrderTime
   virtual UINT      TypeTime(void) const=0;
   virtual MTAPIRES  TypeTime(const UINT type)=0;
   //--- order price
   virtual double    PriceOrder(void) const=0;
   virtual MTAPIRES  PriceOrder(const double price)=0;
   //--- order trigger price (stop-limit price)
   virtual double    PriceTrigger(void) const=0;
   virtual MTAPIRES  PriceTrigger(const double price)=0;
   //--- order current price
   virtual double    PriceCurrent(void) const=0;
   virtual MTAPIRES  PriceCurrent(const double price)=0;
   //--- order SL
   virtual double    PriceSL(void) const=0;
   virtual MTAPIRES  PriceSL(const double price)=0;
   //--- order TP
   virtual double    PriceTP(void) const=0;
   virtual MTAPIRES  PriceTP(const double price)=0;
   //--- order initial volume
   virtual UINT64    VolumeInitial(void) const=0;
   virtual MTAPIRES  VolumeInitial(const UINT64 volume)=0;
   //--- order current volume
   virtual UINT64    VolumeCurrent(void) const=0;
   virtual MTAPIRES  VolumeCurrent(const UINT64 volume)=0;
   //--- expert id (filled by expert advisor)
   virtual UINT64    ExpertID(void) const=0;
   virtual MTAPIRES  ExpertID(const UINT64 id)=0;
   //--- position id
   virtual UINT64    PositionID(void) const=0;
   virtual MTAPIRES  PositionID(const UINT64 id)=0;
   //--- order comment
   virtual LPCWSTR   Comment(void) const=0;
   virtual MTAPIRES  Comment(LPCWSTR comment)=0;
   //--- order activation state, time and price
   virtual UINT      ActivationMode(void) const=0;
   virtual INT64     ActivationTime(void) const=0;
   virtual double    ActivationPrice(void) const=0;
   virtual UINT      ActivationFlags(void) const=0;
   //--- order internal data for API usage
   virtual MTAPIRES  ApiDataSet(const USHORT app_id,const UCHAR id,const INT64 value)=0;
   virtual MTAPIRES  ApiDataSet(const USHORT app_id,const UCHAR id,const UINT64 value)=0;
   virtual MTAPIRES  ApiDataSet(const USHORT app_id,const UCHAR id,const double value)=0;
   virtual MTAPIRES  ApiDataGet(const USHORT app_id,const UCHAR id,INT64& value) const=0;
   virtual MTAPIRES  ApiDataGet(const USHORT app_id,const UCHAR id,UINT64& value) const=0;
   virtual MTAPIRES  ApiDataGet(const USHORT app_id,const UCHAR id,double& value) const=0;
   virtual MTAPIRES  ApiDataClear(const USHORT app_id)=0;
   virtual MTAPIRES  ApiDataClearAll(void)=0;
   //--- order setup time in msc since 1970.01.01
   virtual INT64     TimeSetupMsc(void) const=0;
   virtual MTAPIRES  TimeSetupMsc(const INT64 time)=0;
   //--- order setup time in msc since 1970.01.01
   virtual INT64     TimeDoneMsc(void) const=0;
   virtual MTAPIRES  TimeDoneMsc(const INT64 time)=0;
   //--- order activation state, time and price
   virtual MTAPIRES  ActivationMode(const UINT mode)=0;
   virtual MTAPIRES  ActivationTime(const INT64 atm)=0;
   virtual MTAPIRES  ActivationPrice(const double price)=0;
   virtual MTAPIRES  ActivationFlags(const UINT flags)=0;
   //--- margin conversion rate (from symbol margin currency to deposit currency)
   virtual double    RateMargin(void) const=0;
   virtual MTAPIRES  RateMargin(const double rate)=0;
   //--- user record internal data for API usage
   virtual MTAPIRES  ApiDataUpdate(const UINT pos,const USHORT app_id,const UCHAR id,const INT64 value)=0;
   virtual MTAPIRES  ApiDataUpdate(const UINT pos,const USHORT app_id,const UCHAR id,const UINT64 value)=0;
   virtual MTAPIRES  ApiDataUpdate(const UINT pos,const USHORT app_id,const UCHAR id,const double value)=0;
   virtual MTAPIRES  ApiDataNext(const UINT pos,USHORT& app_id,UCHAR& id,INT64& value) const=0;
   virtual MTAPIRES  ApiDataNext(const UINT pos,USHORT& app_id,UCHAR& id,UINT64& value) const=0;
   virtual MTAPIRES  ApiDataNext(const UINT pos,USHORT& app_id,UCHAR& id,double& value) const=0;
   //--- order ticket
   virtual MTAPIRES  OrderSet(const UINT64 order)=0;
   //--- position by id
   virtual UINT64    PositionByID(void) const=0;
   virtual MTAPIRES  PositionByID(const UINT64 id)=0;
   //--- modification flags
   virtual UINT      ModificationFlags(void) const=0;
   //--- EnOrderState
   virtual MTAPIRES  StateSet(const UINT state)=0;
   //--- EnOrderReason
   virtual MTAPIRES  ReasonSet(const UINT reason)=0;
   //--- order initial volume with extended accuracy
   virtual UINT64    VolumeInitialExt(void) const=0;
   virtual MTAPIRES  VolumeInitialExt(const UINT64 volume)=0;
   //--- order current volume with extended accuracy
   virtual UINT64    VolumeCurrentExt(void) const=0;
   virtual MTAPIRES  VolumeCurrentExt(const UINT64 volume)=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTOrder(void) {}
  };

class IMTOrderArray
  {
public:
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTOrderArray* array)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- add
   virtual MTAPIRES  Add(IMTOrder* order)=0;
   virtual MTAPIRES  AddCopy(const IMTOrder* order)=0;
   virtual MTAPIRES  Add(IMTOrderArray* array)=0;
   virtual MTAPIRES  AddCopy(const IMTOrderArray* array)=0;
   //--- delete & detach
   virtual MTAPIRES  Delete(const UINT pos)=0;
   virtual IMTOrder* Detach(const UINT pos)=0;
   //--- update
   virtual MTAPIRES  Update(const UINT pos,IMTOrder* order)=0;
   virtual MTAPIRES  UpdateCopy(const UINT pos,const IMTOrder* order)=0;
   virtual MTAPIRES  Shift(const UINT pos,const int shift)=0;
   //--- data access
   virtual UINT      Total(void) const=0;
   virtual IMTOrder* Next(const UINT index) const=0;
   //--- sorting and search
   virtual MTAPIRES  Sort(MTSortFunctionPtr sort_function)=0;
   virtual int       Search(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchGreatOrEq(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchGreater(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchLessOrEq(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchLess(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchLeft(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchRight(const void *key,MTSortFunctionPtr sort_function) const=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTOrderArray(void) {}
  };

class IMTOrderSink
  {
public:
   virtual void      OnOrderAdd(const IMTOrder*    /*order*/)       {  }
   virtual void      OnOrderUpdate(const IMTOrder* /*order*/)       {  }
   virtual void      OnOrderDelete(const IMTOrder* /*order*/)       {  }
   virtual void      OnOrderClean(const UINT64 /*login*/)           {  }
   virtual void      OnOrderSync(void)                              {  }
  };

class IMTHistorySink
  {
public:
   virtual void      OnHistoryAdd(const IMTOrder*    /*order*/)     {  }
   virtual void      OnHistoryUpdate(const IMTOrder* /*order*/)     {  }
   virtual void      OnHistoryDelete(const IMTOrder* /*order*/)     {  }
   virtual void      OnHistoryClean(const UINT64 /*login*/)         {  }
   virtual void      OnHistorySync(void)                            {  }
  };

class IMTDaily
  {
public:
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTDaily* exec)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- date and time
   virtual INT64     Datetime(void) const=0;
   virtual MTAPIRES  Datetime(const INT64 datetime)=0;
   //--- previous generation datetime
   virtual INT64     DatetimePrev(void) const=0;
   virtual MTAPIRES  DatetimePrev(const INT64 datetime)=0;
   //--- login
   virtual UINT64    Login(void) const=0;
   virtual MTAPIRES  Login(const UINT64 login)=0;
   //--- name
   virtual LPCWSTR   Name(void) const=0;
   virtual MTAPIRES  Name(LPCWSTR name)=0;
   //--- group
   virtual LPCWSTR   Group(void) const=0;
   virtual MTAPIRES  Group(LPCWSTR group)=0;
   //--- currency
   virtual LPCWSTR   Currency(void) const=0;
   virtual MTAPIRES  Currency(LPCWSTR curr)=0;
   virtual UINT      CurrencyDigits(void) const=0;
   //--- company
   virtual LPCWSTR   Company(void) const=0;
   virtual MTAPIRES  Company(LPCWSTR company)=0;
   //--- e-mail
   virtual LPCWSTR   EMail(void) const=0;
   virtual MTAPIRES  EMail(LPCWSTR mail)=0;
   //--- balance
   virtual double    Balance(void) const=0;
   virtual MTAPIRES  Balance(const double balance)=0;
   //--- credit
   virtual double    Credit(void) const=0;
   virtual MTAPIRES  Credit(const double credit)=0;
   //--- interest rate
   virtual double    InterestRate(void) const=0;
   virtual MTAPIRES  InterestRate(const double credit)=0;
   //--- commission daily
   virtual double    CommissionDaily(void) const=0;
   virtual MTAPIRES  CommissionDaily(const double comm)=0;
   //--- commission monthly
   virtual double    CommissionMonthly(void) const=0;
   virtual MTAPIRES  CommissionMonthly(const double comm)=0;
   //--- commission daily
   virtual double    AgentDaily(void) const=0;
   virtual MTAPIRES  AgentDaily(const double agent)=0;
   //--- commission monthly
   virtual double    AgentMonthly(void) const=0;
   virtual MTAPIRES  AgentMonthly(const double agent)=0;
   //--- last day balance
   virtual double    BalancePrevDay(void) const=0;
   virtual MTAPIRES  BalancePrevDay(const double balance)=0;
   //--- last month balance
   virtual double    BalancePrevMonth(void) const=0;
   virtual MTAPIRES  BalancePrevMonth(const double balance)=0;
   //--- last day equity
   virtual double    EquityPrevDay(void) const=0;
   virtual MTAPIRES  EquityPrevDay(const double balance)=0;
   //--- last month equity
   virtual double    EquityPrevMonth(void) const=0;
   virtual MTAPIRES  EquityPrevMonth(const double balance)=0;
   //---
   //--- margin
   //---
   virtual double    Margin(void) const=0;
   virtual MTAPIRES  Margin(const double margin)=0;
   //--- free margin 
   virtual double    MarginFree(void) const=0;
   virtual MTAPIRES  MarginFree(const double margin_free)=0;
   //--- margin level
   virtual double    MarginLevel(void) const=0;
   virtual MTAPIRES  MarginLevel(const double margin_level)=0;
   //--- margin leverage
   virtual UINT      MarginLeverage(void) const=0;
   virtual MTAPIRES  MarginLeverage(const UINT leverage)=0;
   //---
   //--- floating profit
   //---
   virtual double    Profit(void) const=0;
   virtual MTAPIRES  Profit(const double profit)=0;
   //--- storage
   virtual double    ProfitStorage(void) const=0;
   virtual MTAPIRES  ProfitStorage(const double storage)=0;
   //--- obsolete value
   virtual double    ObsoleteValue(void) const=0;
   virtual MTAPIRES  ObsoleteValue(const double value)=0;
   //--- equity
   virtual double    ProfitEquity(void) const=0;
   virtual MTAPIRES  ProfitEquity(const double equity)=0;
   //---
   //--- daily fixed profit details
   //---
   virtual double    DailyProfit(void) const=0;
   virtual MTAPIRES  DailyProfit(const double profit)=0;
   //---
   virtual double    DailyBalance(void) const=0;
   virtual MTAPIRES  DailyBalance(const double balance)=0;
   //---
   virtual double    DailyCredit(void) const=0;
   virtual MTAPIRES  DailyCredit(const double comm)=0;
   //---
   virtual double    DailyCharge(void) const=0;
   virtual MTAPIRES  DailyCharge(const double charge)=0;
   //---
   virtual double    DailyCorrection(void) const=0;
   virtual MTAPIRES  DailyCorrection(const double correction)=0;
   //---
   virtual double    DailyBonus(void) const=0;
   virtual MTAPIRES  DailyBonus(const double bonus)=0;
   //---
   virtual double    DailyStorage(void) const=0;
   virtual MTAPIRES  DailyStorage(const double storage)=0;
   //---
   virtual double    DailyCommInstant(void) const=0;
   virtual MTAPIRES  DailyCommInstant(const double comm)=0;
   //---
   virtual double    DailyCommRound(void) const=0;
   virtual MTAPIRES  DailyCommRound(const double comm)=0;
   //---
   virtual double    DailyAgent(void) const=0;
   virtual MTAPIRES  DailyAgent(const double comm)=0;
   //---
   virtual double    DailyInterest(void) const=0;
   virtual MTAPIRES  DailyInterest(const double interest)=0;
   //--- list of open positions
   virtual MTAPIRES  PositionAdd(IMTPosition* position)=0;
   virtual MTAPIRES  PositionUpdate(const UINT pos,const IMTPosition* position)=0;
   virtual MTAPIRES  PositionDelete(const UINT pos)=0;
   virtual MTAPIRES  PositionClear(void)=0;
   virtual MTAPIRES  PositionShift(const UINT pos,const int shift)=0;
   virtual UINT      PositionTotal(void) const=0;
   virtual MTAPIRES  PositionNext(const UINT pos,IMTPosition* position) const=0;
   virtual MTAPIRES  PositionGet(LPCWSTR symbol,IMTPosition* position) const=0;
   //--- list of open orders
   virtual MTAPIRES  OrderAdd(IMTOrder* order)=0;
   virtual MTAPIRES  OrderUpdate(const UINT pos,const IMTOrder* order)=0;
   virtual MTAPIRES  OrderDelete(const UINT pos)=0;
   virtual MTAPIRES  OrderClear(void)=0;
   virtual MTAPIRES  OrderShift(const UINT pos,const int shift)=0;
   virtual UINT      OrderTotal(void) const=0;
   virtual MTAPIRES  OrderNext(const UINT pos,IMTOrder* order) const=0;
   virtual MTAPIRES  OrderGet(const UINT64 ticket,IMTOrder* order) const=0;
   //--- assets
   virtual double    ProfitAssets(void) const=0;
   virtual MTAPIRES  ProfitAssets(const double assets)=0;
   //--- liabilities
   virtual double    ProfitLiabilities(void) const=0;
   virtual MTAPIRES  ProfitLiabilities(const double liabilities)=0;
   //--- 
   virtual double    DailyDividend(void) const=0;
   virtual MTAPIRES  DailyDividend(const double dividend)=0;
   //--- 
   virtual double    DailyTaxes(void) const=0;
   virtual MTAPIRES  DailyTaxes(const double taxes)=0;
   //--- 
   virtual double    DailySOCompensation(void) const=0;
   virtual MTAPIRES  DailySOCompensation(const double compensation)=0;
   //--- 
   virtual double    DailyCommFee(void) const=0;
   virtual MTAPIRES  DailyCommFee(const double fee)=0;
   //--- 
   virtual double    DailySOCompensationCredit(void) const=0;
   virtual MTAPIRES  DailySOCompensationCredit(const double compensation)=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTDaily(void) {}
  };

class IMTDailyArray
  {
public:
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTDailyArray* array)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- add
   virtual MTAPIRES  Add(IMTDaily* daily)=0;
   virtual MTAPIRES  AddCopy(const IMTDaily* daily)=0;
   virtual MTAPIRES  Add(IMTDailyArray* array)=0;
   virtual MTAPIRES  AddCopy(const IMTDailyArray* array)=0;
   //--- delete & detach
   virtual MTAPIRES  Delete(const UINT pos)=0;
   virtual IMTDaily* Detach(const UINT pos)=0;
   //--- update
   virtual MTAPIRES  Update(const UINT pos,IMTDaily* daily)=0;
   virtual MTAPIRES  UpdateCopy(const UINT pos,const IMTDaily* daily)=0;
   virtual MTAPIRES  Shift(const UINT pos,const int shift)=0;
   //--- data access
   virtual UINT      Total(void) const=0;
   virtual IMTDaily* Next(const UINT index) const=0;
   //--- sorting and search
   virtual MTAPIRES  Sort(MTSortFunctionPtr sort_function)=0;
   virtual int       Search(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchGreatOrEq(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchGreater(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchLessOrEq(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchLess(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchLeft(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchRight(const void *key,MTSortFunctionPtr sort_function) const=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTDailyArray(void) {}
  };

class IMTDailySink
  {
public:
   virtual void      OnDailyAdd(const IMTDaily*    /*daily*/)  {  }
   virtual void      OnDailyUpdate(const IMTDaily* /*daily*/)  {  }
   virtual void      OnDailyDelete(const IMTDaily* /*daily*/)  {  }
   virtual void      OnDailyClean(const UINT64 /*login*/)      {  }
   virtual void      OnDailySync(void)                         {  }
  };

enum EnMTFeederConstants
  {
   MT_FEEDER_DEALER =-1,                              // feeder index for dealer ticks 
   MT_FEEDER_OFFSET =64                               // index offset for datafeed ticks
  };

#pragma pack(push,1)
struct MTTick
  {
   //--- tick flags
   enum EnTickFlags
     {
      TICK_FLAG_BUY   =1,                             // tick created due buy operation
      TICK_FLAG_SELL  =2,                             // tick created due sell operation
      //--- enumeration borders
      TICK_FLAG_NONE  =0,                             // none
      TICK_FLAG_ALL   =TICK_FLAG_BUY|TICK_FLAG_SELL   // all flags
     };
   wchar_t           symbol[32];                      // symbol
   wchar_t           bank[32];                        // tick source ticker
   INT64             datetime;                        // datetime in seconds since 01/01/1970
   //--- basic price data
   double            bid;                             // bid price
   double            ask;                             // ask price
   double            last;                            // last price
   UINT64            volume;                          // last trade volume
   //--- additional data
   INT64             datetime_msc;                    // datetime in milliseconds since 01/01/1970
   UINT64            flags;                           // flags
   UINT64            volume_ext;                      // last trade volume with extended accuracy
   //--- reserved data
   UINT              reserved[26];                    // reserved
  };
#pragma pack(pop)

#pragma pack(push,1)
struct MTTickShort
  {
   //--- tick flags
   enum EnTickShortFlags
     {
      TICK_SHORT_FLAG_RAW   =0x00000001,              // raw tick 
      TICK_SHORT_FLAG_BID   =0x00000002,              // tick changes bid price value
      TICK_SHORT_FLAG_ASK   =0x00000004,              // tick changes ask price value
      TICK_SHORT_FLAG_LAST  =0x00000008,              // tick changes last price value
      TICK_SHORT_FLAG_VOLUME=0x00000010,              // tick changes volume value
      TICK_SHORT_FLAG_BUY   =0x00000020,              // tick created due buy operation
      TICK_SHORT_FLAG_SELL  =0x00000040,              // tick created due sell operation
      //--- enumeration borders
      TICK_SHORT_FLAG_NONE  =0x00000000,              // none
     };
   INT64             datetime;                        // last update datetime in seconds since 01/01/1970
   double            bid;                             // bid price
   double            ask;                             // ask price
   double            last;                            // last price
   UINT64            volume;                          // last trade volume
   INT64             datetime_msc;                    // datetime in milliseconds since 01/01/1970
   UINT64            flags;                           // flags
   UINT64            volume_ext;                      // last trade volume with extended volume
   UINT              reserved[26];                    // reserved
  };
#pragma pack(pop)

#pragma pack(push,1)
struct MTTickRate
  {
   //--- tick flags
   enum EnTickShortFlags
     {
      TICK_SHORT_FLAG_RAW   =0x00000001,              // raw tick 
      TICK_SHORT_FLAG_BID   =0x00000002,              // tick changes bid price value
      TICK_SHORT_FLAG_ASK   =0x00000004,              // tick changes ask price value
      TICK_SHORT_FLAG_LAST  =0x00000008,              // tick changes last price value
      TICK_SHORT_FLAG_VOLUME=0x00000010,              // tick changes volume value
      TICK_SHORT_FLAG_BUY   =0x00000020,              // tick created due buy operation
      TICK_SHORT_FLAG_SELL  =0x00000040,              // tick created due sell operation
      //--- enumeration borders
      TICK_SHORT_FLAG_NONE  =0x00000000,              // none
     };
   INT64             datetime_msc;                    // datetime in milliseconds since 01/01/1970
   double            bid;                             // bid price
   double            ask;                             // ask price
   double            last;                            // last price
   UINT64            flags;                           // flags
   UINT64            volume_ext;                      // last trade volume with extended volume
   INT64             reserved[2];
  };
#pragma pack(pop)

#pragma pack(push,1)
struct MTTickStat
  {
   wchar_t           symbol[32];                   // symbol
   INT64             datetime;                     // last update datetime in seconds since 01/01/1970
   //--- bid
   double            bid_high;                     // bid high
   double            bid_low;                      // bid low
   //--- ask
   double            ask_high;                     // ask high
   double            ask_low;                      // ask low
   //--- last
   double            last_high;                    // last high
   double            last_low;                     // last low
   //--- trade volume
   UINT64            vol_high;                     // last high
   UINT64            vol_low;                      // last low
   //--- trade session  statistics
   UINT64            trade_deals;                  // total deals in current session
   UINT64            trade_volume;                 // total deals volume in current session
   UINT64            trade_turnover;               // total turnover in current session
   UINT64            trade_interest;               // open interest in current session
   UINT64            trade_buy_orders;             // total buy orders
   UINT64            trade_buy_volume;             // total buy orders volume
   UINT64            trade_sell_orders;            // total sell orders
   UINT64            trade_sell_volume;            // total sell orders volume
   UINT64            trade_volume_ext;             // total deals volume in current session with extended accuracy
   UINT64            trade_buy_volume_ext;         // total buy orders volume with extended accuracy
   UINT64            trade_sell_volume_ext;        // total sell orders volume with extended accuracy
   UINT64            vol_high_ext;                 // last high with extended accuracy
   UINT64            vol_low_ext;                  // last low with extended accuracy
   int               trade_reserved[20];           // reserved
   //--- datetime
   INT64             datetime_msc;                 // last update datetime in milliseconds since 01/01/1970
   //--- price session  statistics
   double            price_open;                   // session open price
   double            price_close;                  // session close price
   double            price_aw;                     // average weighted price
   double            price_obsolete;               // obsolete field, doesn't accepted by server, ex "price change percentage"
   double            price_volatility;             // price volatility
   double            price_theoretical;            // theoretical price
   double            price_greeks_delta;           // option\warrant delta
   double            price_greeks_theta;           // option\warrant theta
   double            price_greeks_gamma;           // option\warrant gamma
   double            price_greeks_vega;            // option\warrant vega
   double            price_greeks_rho;             // option\warrant rho
   double            price_greeks_omega;           // option\warrant omega
   double            price_sensitivity;            // option\warrant sensitivity
   int               price_reserved[14];           // reserved
  };
#pragma pack(pop)

class IMTTickSink
  {
public:
   //--- tick events
   virtual void      OnTick(LPCWSTR /*symbol*/,const MTTickShort& /*tick*/)       { }
   virtual void      OnTickStat(const MTTickStat& /*tick*/)                       { }
   //--- tick hooks
   virtual MTAPIRES  HookTick(const int /*feeder*/,MTTick& /*tick*/)              { return(MT_RET_OK); }
   virtual MTAPIRES  HookTickStat(const int /*feeder*/,MTTickStat& /*tstat*/)     { return(MT_RET_OK); }
   //--- extended tick events
   virtual void      OnTick(const int /*feeder*/,const MTTick& /*tick*/)          { }
   virtual void      OnTickStat(const int /*feeder*/,const MTTickStat& /*tstat*/) { }
  };

#pragma pack(push,1)
struct MTMailRange
  {
   UINT64            first_login;
   UINT64            last_login;
   UINT              reserved[4];
  };
#pragma pack(pop)

class IMTMail
  {
public:
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTMail* mail)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- mail ID and parent mail ID
   virtual UINT64    ID(void) const=0;
   virtual UINT64    Parent(void) const=0;
   //--- subject
   virtual LPCWSTR   Subject(void) const=0;
   virtual MTAPIRES  Subject(LPCWSTR subject)=0;
   //--- from ID
   virtual UINT64    From(void) const=0;
   virtual MTAPIRES  From(const UINT64 id)=0;
   //--- from name
   virtual LPCWSTR   FromName(void) const=0;
   virtual MTAPIRES  FromName(LPCWSTR name)=0;
   //--- to ID
   virtual UINT64    To(void) const=0;
   virtual MTAPIRES  To(const UINT64 id)=0;
   //--- to name
   virtual LPCWSTR   ToName(void) const=0;
   virtual MTAPIRES  ToName(LPCWSTR name)=0;
   //--- to ID ranges
   virtual MTAPIRES  ToRangesAdd(MTMailRange& range)=0;
   virtual MTAPIRES  ToRangesDelete(const UINT pos)=0;
   virtual MTAPIRES  ToRangesClear(void)=0;
   virtual UINT      ToRangesTotal(void) const=0;
   virtual MTAPIRES  ToRangesNext(const UINT pos,MTMailRange& range) const=0;
   //--- time
   virtual INT64     Time(void) const=0;
   //--- body
   virtual LPCVOID   Body(void) const=0;
   virtual UINT      BodySize(void) const=0;
   virtual MTAPIRES  Body(LPCVOID body,const UINT body_size)=0;
   //--- attachments
   virtual MTAPIRES  AttachmentsAdd(LPCWSTR filename,LPCVOID attachment,const UINT attachment_size)=0;
   virtual MTAPIRES  AttachmentsClear(void)=0;
   virtual UINT      AttachmentsTotal(void) const=0;
   virtual LPVOID    AttachmentsBody(const UINT pos) const=0;
   virtual UINT      AttachmentsSize(const UINT pos) const=0;
   virtual LPCWSTR   AttachmentsName(const UINT pos) const=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTMail(void) {}
  };

class IMTMailSink
  {
public:
   virtual void      OnMail(const IMTMail* /*mail*/) {                    };
   virtual MTAPIRES  HookMail(IMTMail* /*mail*/)     { return(MT_RET_OK); }
  };

class IMTNews
  {
public:
   //--- news flags
   enum EnNewsFlags
     {
      NEWS_FLAGS_NONE      =0x0000,
      NEWS_FLAGS_PRIORITY  =0x0001,
      NEWS_FLAGS_READ      =0x0002,
      NEWS_FLAGS_NOBODY    =0x0004,
      NEWS_FLAGS_CALENDAR  =0x0008,
      //--- enumeration borders
      NEWS_FLAGS_FIRST        =NEWS_FLAGS_PRIORITY,
      NEWS_FLAGS_ALL          =NEWS_FLAGS_PRIORITY|NEWS_FLAGS_READ|NEWS_FLAGS_NOBODY|NEWS_FLAGS_CALENDAR
     };

public:
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTNews* news)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- news ID
   virtual UINT64    ID(void) const=0;
   //--- subject
   virtual LPCWSTR   Subject(void) const=0;
   virtual MTAPIRES  Subject(LPCWSTR subject)=0;
   //--- category
   virtual LPCWSTR   Category(void) const=0;
   virtual MTAPIRES  Category(LPCWSTR category)=0;
   //--- time
   virtual INT64     Time(void) const=0;
   virtual MTAPIRES  Time(const INT64 datetime)=0;
   //--- language
   virtual UINT      Language(void) const=0;
   virtual MTAPIRES  Language(const UINT language)=0;
   //--- language
   virtual UINT      Flags(void) const=0;
   virtual MTAPIRES  Flags(const UINT flags)=0;
   //--- body
   virtual LPCVOID   Body(void) const=0;
   virtual UINT      BodySize(void) const=0;
   virtual MTAPIRES  Body(LPCVOID body,const UINT body_size)=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTNews(void) {}
  };

class IMTNewsSink
  {
public:
   virtual void      OnNews(const IMTNews* /*news*/)                  {                    };
   virtual MTAPIRES  HookNews(const int /*feeder*/,IMTNews* /*news*/) { return(MT_RET_OK); }
  };

class IMTByteStream
  {
public:
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTByteStream* stream)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- common properties
   virtual UINT      Len(void)=0;
   virtual UINT      ReadLen(void)=0;
   //--- data add
   virtual MTAPIRES  Add(const void *buf,const UINT len)=0;
   virtual MTAPIRES  AddChar(const char data)=0;
   virtual MTAPIRES  AddUChar(const UCHAR data)=0;
   virtual MTAPIRES  AddShort(const short data)=0;
   virtual MTAPIRES  AddUShort(const USHORT data)=0;
   virtual MTAPIRES  AddInt(const int data)=0;
   virtual MTAPIRES  AddUInt(const UINT data)=0;
   virtual MTAPIRES  AddInt64(const INT64 data)=0;
   virtual MTAPIRES  AddUInt64(const UINT64 data)=0;
   virtual MTAPIRES  AddFloat(const float data)=0;
   virtual MTAPIRES  AddDouble(const double data)=0;
   virtual MTAPIRES  AddResult(const MTAPIRES data)=0;
   virtual MTAPIRES  AddStr(LPCWSTR buf)=0;
   //--- data read
   virtual void      ReadReset(void)=0;
   virtual MTAPIRES  Read(void* buf,const UINT len)=0;
   virtual MTAPIRES  ReadSkip(const UINT len)=0;
   virtual MTAPIRES  ReadChar(char& data)=0;
   virtual MTAPIRES  ReadUChar(UCHAR& data)=0;
   virtual MTAPIRES  ReadShort(SHORT& data)=0;
   virtual MTAPIRES  ReadUShort(USHORT& data)=0;
   virtual MTAPIRES  ReadInt(int& data)=0;
   virtual MTAPIRES  ReadUInt(UINT& data)=0;
   virtual MTAPIRES  ReadInt64(INT64& data)=0;
   virtual MTAPIRES  ReadUInt64(UINT64& data)=0;
   virtual MTAPIRES  ReadFloat(float& data)=0;
   virtual MTAPIRES  ReadDouble(double& data)=0;
   virtual MTAPIRES  ReadResult(MTAPIRES& data)=0;
   virtual MTAPIRES  ReadStr(MTAPISTR& buf)=0;
   //--- web api data add
   virtual MTAPIRES  WebAddParamStr(LPCWSTR name,LPCWSTR value)=0;
   virtual MTAPIRES  WebAddParamChar(LPCWSTR name,const char value)=0;
   virtual MTAPIRES  WebAddParamUChar(LPCWSTR name,const UCHAR value)=0;
   virtual MTAPIRES  WebAddParamShort(LPCWSTR name,const short value)=0;
   virtual MTAPIRES  WebAddParamUShort(LPCWSTR name,const USHORT value)=0;
   virtual MTAPIRES  WebAddParamInt(LPCWSTR name,const int value)=0;
   virtual MTAPIRES  WebAddParamUInt(LPCWSTR name,const UINT value)=0;
   virtual MTAPIRES  WebAddParamInt64(LPCWSTR name,const INT64 value)=0;
   virtual MTAPIRES  WebAddParamUInt64(LPCWSTR name,const UINT64 value)=0;
   virtual MTAPIRES  WebAddParamDouble(LPCWSTR name,const double value,const UINT digits)=0;
   virtual MTAPIRES  WebAddParamFinalize(void)=0;
   //--- web api data read
   virtual MTAPIRES  WebReadCommand(MTAPISTR& cmd)=0;
   virtual MTAPIRES  WebReadParamName(MTAPISTR& name)=0;
   virtual MTAPIRES  WebReadParamStr(MTAPISTR& str)=0;
   virtual MTAPIRES  WebReadParamStr(LPWSTR value,const UINT size)=0;
   virtual MTAPIRES  WebReadParamSkip(void)=0;
   virtual MTAPIRES  WebReadParamChar(char& data)=0;
   virtual MTAPIRES  WebReadParamUChar(UCHAR& data)=0;
   virtual MTAPIRES  WebReadParamShort(SHORT& data)=0;
   virtual MTAPIRES  WebReadParamUShort(USHORT& data)=0;
   virtual MTAPIRES  WebReadParamInt(int& value)=0;
   virtual MTAPIRES  WebReadParamUInt(UINT& value)=0;
   virtual MTAPIRES  WebReadParamInt64(INT64& value)=0;
   virtual MTAPIRES  WebReadParamUInt64(UINT64& value)=0;
   virtual MTAPIRES  WebReadParamDouble(double& value)=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTByteStream(void) {}
  };

class IMTCustomSink
  {
public:
   //--- manager API command
   virtual MTAPIRES  HookManagerCommand(LPCWSTR              /*ip*/,
                                        const IMTConManager* /*manager*/,
                                        LPCVOID              /*indata*/,
                                        const UINT           /*indata_len*/,
                                        LPVOID&              /*outdata*/,
                                        UINT&                /*outdata_len*/) { return(MT_RET_OK_NONE); }
   //--- manager API command
   virtual MTAPIRES  HookManagerCommand(const UINT64         /*session*/,
                                        LPCWSTR              /*ip*/,
                                        const IMTConManager* /*manager*/,
                                        IMTByteStream*       /*indata*/,
                                        IMTByteStream*       /*outdata*/)     { return(MT_RET_OK_NONE); }
   //--- Web API command
   virtual MTAPIRES  HookWebAPICommand(const UINT64          /*session*/,
                                       LPCWSTR               /*ip*/,
                                       const IMTConManager*  /*manager*/,
                                       LPCWSTR               /*command*/,
                                       IMTByteStream*        /*indata*/,
                                       IMTByteStream*        /*outdata*/)     { return(MT_RET_OK_NONE); }
  };

class IMTRequest
  {
public:
   //--- trade action types
   enum EnTradeActions
     {
      //--- clients actions
      TA_CLIENT_FIRST       =0,
      TA_PRICE              =0,
      TA_REQUEST            =1,
      TA_INSTANT            =2,
      TA_MARKET             =3,
      TA_EXCHANGE           =4,
      TA_PENDING            =5,
      TA_SLTP               =6,
      TA_MODIFY             =7,
      TA_REMOVE             =8,
      TA_TRANSFER           =9,
      TA_CLOSE_BY           =10,
      TA_CLIENT_LAST        =TA_CLOSE_BY,
      //--- server actions
      TA_SERVER_FIRST       =100,
      TA_ACTIVATE           =100,
      TA_ACTIVATE_SL        =101,
      TA_ACTIVATE_TP        =102,
      TA_ACTIVATE_STOPLIMIT =103,
      TA_STOPOUT_ORDER      =104,
      TA_STOPOUT_POSITION   =105,
      TA_EXPIRATION         =106,
      TA_SERVER_LAST        =TA_EXPIRATION,
      //--- dealer actions
      TA_DEALER_FIRST       =200,
      TA_DEALER_POS_EXECUTE =200,
      TA_DEALER_ORD_PENDING =201,
      TA_DEALER_POS_MODIFY  =202,
      TA_DEALER_ORD_MODIFY  =203,
      TA_DEALER_ORD_REMOVE  =204,
      TA_DEALER_ORD_ACTIVATE=205,
      TA_DEALER_BALANCE     =206,
      TA_DEALER_ORD_SLIMIT  =207,
      TA_DEALER_CLOSE_BY    =208,
      TA_DEALER_LAST        =TA_DEALER_CLOSE_BY,
      //--- enumeration borders
      TA_FIRST              =TA_PRICE,
      TA_LAST               =TA_DEALER_CLOSE_BY,
      TA_END                =255
     };
   //--- trade action flags
   enum EnTradeActionFlags
     {
      TA_FLAG_CLOSE            =0x00000001, // position close request
      TA_FLAG_MARKET           =0x00000002, // market prices request
      TA_FLAG_CHANGED_PRICE    =0x00000004, // order price or position open price will be changed
      TA_FLAG_CHANGED_TRIGGER  =0x00000008, // order trigger price will be changed
      TA_FLAG_CHANGED_SL       =0x00000010, // order or position SL will be changed
      TA_FLAG_CHANGED_TP       =0x00000020, // order or position TP will be changed
      TA_FLAG_CHANGED_EXP_TYPE =0x00000040, // order expiration type will be changed
      TA_FLAG_CHANGED_EXP_TIME =0x00000080, // order expiration datetime will be changed
      TA_FLAG_EXPERT           =0x00000100, // request from expert
      TA_FLAG_SIGNAL           =0x00000200, // request from signal
      TA_FLAG_SKIP_MARGIN_CHECK=0x00000400, // skip margin check (only for dealers)
      //--- enumeration borders
      TA_FLAG_NONE            =0x00000000,
      TA_FLAG_ALL             =TA_FLAG_CLOSE      | TA_FLAG_MARKET     | TA_FLAG_CHANGED_PRICE | TA_FLAG_CHANGED_TRIGGER |
      TA_FLAG_CHANGED_SL | TA_FLAG_CHANGED_TP | TA_FLAG_CHANGED_EXP_TYPE | TA_FLAG_CHANGED_EXP_TIME | TA_FLAG_EXPERT | TA_FLAG_SIGNAL |
      TA_FLAG_SKIP_MARGIN_CHECK
     };
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTRequest* request)=0;
   virtual MTAPIRES  Clear(void)=0;
   virtual LPCWSTR   Print(MTAPISTR& string) const=0;
   //--- request id
   virtual UINT      ID(void) const=0;
   //--- client login
   virtual UINT64    Login(void) const=0;
   virtual MTAPIRES  Login(const UINT64 login)=0;
   //--- client group
   virtual LPCWSTR   Group(void) const=0;
   //--- symbol
   virtual LPCWSTR   Symbol(void) const=0;
   virtual MTAPIRES  Symbol(LPCWSTR symbol)=0;
   //--- price digits
   virtual UINT      Digits(void) const=0;
   //--- EnTradeActions
   virtual UINT      Action(void) const=0;
   virtual MTAPIRES  Action(const UINT action)=0;
   //--- order expiration
   virtual INT64     TimeExpiration(void) const=0;
   virtual MTAPIRES  TimeExpiration(const INT64 time)=0;
   //--- order type IMTOrder::EnOrderType
   virtual UINT      Type(void) const=0;
   virtual MTAPIRES  Type(const UINT type)=0;
   //--- IMTOrder::EnOrderFilling
   virtual UINT      TypeFill(void) const=0;
   virtual MTAPIRES  TypeFill(const UINT type)=0;
   //--- IMTOrder::EnOrderTime
   virtual UINT      TypeTime(void) const=0;
   virtual MTAPIRES  TypeTime(const UINT type)=0;
   //--- additional flags EnTradeActionFlags
   virtual UINT64    Flags(void) const=0;
   virtual MTAPIRES  Flags(const UINT64 flags)=0;
   //--- volume
   virtual UINT64    Volume(void) const=0;
   virtual MTAPIRES  Volume(const UINT64 volume)=0;
   //--- order ticket
   virtual UINT64    Order(void) const=0;
   virtual MTAPIRES  Order(const UINT64 order)=0;
   //--- order ticket in external system (exchange, ECN, etc)
   virtual LPCWSTR   OrderExternalID(void) const=0;
   virtual MTAPIRES  OrderExternalID(LPCWSTR id)=0;
   //--- order price
   virtual double    PriceOrder(void) const=0;
   virtual MTAPIRES  PriceOrder(const double price)=0;
   //--- order trigger price
   virtual double    PriceTrigger(void) const=0;
   virtual MTAPIRES  PriceTrigger(const double price)=0;
   //--- order SL price
   virtual double    PriceSL(void) const=0;
   virtual MTAPIRES  PriceSL(const double price)=0;
   //--- order TP price
   virtual double    PriceTP(void) const=0;
   virtual MTAPIRES  PriceTP(const double price)=0;
   //--- price deviation details
   virtual UINT64    PriceDeviation(void) const=0;
   virtual MTAPIRES  PriceDeviation(const UINT64 deviation)=0;
   virtual double    PriceDeviationTop(void) const=0;
   virtual double    PriceDeviationBottom(void) const=0;
   //--- comment
   virtual LPCWSTR   Comment(void) const=0;
   virtual MTAPIRES  Comment(LPCWSTR comment)=0;
   //---
   //--- request result information
   //---
   //--- request state retcode
   virtual MTAPIRES  ResultRetcode(void) const=0;
   //--- request dealer
   virtual UINT64    ResultDealer(void) const=0;
   //--- request result deal
   virtual UINT64    ResultDeal(void) const=0;
   //--- request result order
   virtual UINT64    ResultOrder(void) const=0;
   //--- request result confirmed volume
   virtual UINT64    ResultVolume(void) const=0;
   //--- request result confirmed price
   virtual double    ResultPrice(void) const=0;
   //--- request result dealer prices
   virtual double    ResultDealerBid(void) const=0;
   virtual double    ResultDealerAsk(void) const=0;
   virtual double    ResultDealerLast(void) const=0;
   //--- request result market prices
   virtual double    ResultMarketBid(void) const=0;
   virtual double    ResultMarketAsk(void) const=0;
   virtual double    ResultMarketLast(void) const=0;
   //--- request result comment
   virtual LPCWSTR   ResultComment(void) const=0;
   //--- external system trade account (exchange, ECN, etc)
   virtual LPCWSTR   ExternalAccount(void) const=0;
   virtual MTAPIRES  ExternalAccount(LPCWSTR account)=0;
   //--- client side request id
   virtual UINT      IDClient(void) const=0;
   //--- source request ip
   virtual LPCWSTR   IP(void) const=0;
   virtual MTAPIRES  IP(LPCWSTR ip)=0;
   //--- source dealer login (for dealer transaction)
   virtual UINT64    SourceLogin(void) const=0;
   virtual MTAPIRES  SourceLogin(const UINT64 login)=0;
   //--- position ticket
   virtual UINT64    Position(void) const=0;
   virtual MTAPIRES  Position(const UINT64 position)=0;
   //--- position ticket for close-by
   virtual UINT64    PositionBy(void) const=0;
   virtual MTAPIRES  PositionBy(const UINT64 position)=0;
   //--- position ticket in external system (exchange, ECN, etc)
   virtual LPCWSTR   PositionExternalID(void) const=0;
   virtual MTAPIRES  PositionExternalID(LPCWSTR id)=0;
   //--- position ticket in external system (exchange, ECN, etc)
   virtual LPCWSTR   PositionByExternalID(void) const=0;
   virtual MTAPIRES  PositionByExternalID(LPCWSTR id)=0;
   //--- volume with extended accuracy
   virtual UINT64    VolumeExt(void) const=0;
   virtual MTAPIRES  VolumeExt(const UINT64 volume)=0;
   //--- request result confirmed volume with extended accuracy
   virtual UINT64    ResultVolumeExt(void) const=0;
   //--- price digits
   virtual MTAPIRES  DigitsSet(const UINT digits)=0;
   //--- request internal data for API usage
   virtual MTAPIRES  ApiDataSet(const USHORT app_id,const UCHAR id,const INT64 value)=0;
   virtual MTAPIRES  ApiDataSet(const USHORT app_id,const UCHAR id,const UINT64 value)=0;
   virtual MTAPIRES  ApiDataSet(const USHORT app_id,const UCHAR id,const double value)=0;
   virtual MTAPIRES  ApiDataGet(const USHORT app_id,const UCHAR id,INT64& value) const=0;
   virtual MTAPIRES  ApiDataGet(const USHORT app_id,const UCHAR id,UINT64& value) const=0;
   virtual MTAPIRES  ApiDataGet(const USHORT app_id,const UCHAR id,double& value) const=0;
   virtual MTAPIRES  ApiDataUpdate(const UINT pos,const USHORT app_id,const UCHAR id,const INT64 value)=0;
   virtual MTAPIRES  ApiDataUpdate(const UINT pos,const USHORT app_id,const UCHAR id,const UINT64 value)=0;
   virtual MTAPIRES  ApiDataUpdate(const UINT pos,const USHORT app_id,const UCHAR id,const double value)=0;
   virtual MTAPIRES  ApiDataNext(const UINT pos,USHORT& app_id,UCHAR& id,INT64& value) const=0;
   virtual MTAPIRES  ApiDataNext(const UINT pos,USHORT& app_id,UCHAR& id,UINT64& value) const=0;
   virtual MTAPIRES  ApiDataNext(const UINT pos,USHORT& app_id,UCHAR& id,double& value) const=0;
   virtual LPVOID    ApiDataRaw(void) const=0;
   virtual UINT      ApiDataRawMax(void) const=0;
   virtual MTAPIRES  ApiDataClear(const USHORT app_id)=0;
   virtual MTAPIRES  ApiDataClearAll(void)=0;
   //--- current order volume 
   virtual UINT64    VolumeCurrent(void) const=0;
   virtual MTAPIRES  VolumeCurrent(const UINT64 volume)=0;
   //--- current order volume 
   virtual UINT64    VolumeCurrentExt(void) const=0;
   virtual MTAPIRES  VolumeCurrentExt(const UINT64 volume)=0;
   //--- trade symbol before symbols mapping
   virtual LPCWSTR   SymbolOriginal(void) const=0;
   virtual MTAPIRES  SymbolOriginal(LPCWSTR symbol)=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTRequest(void) {}
  };

class IMTConfirm
  {
public:
   //--- confirm flags
   enum EnConfirmFlags
     {
      CONFIRM_FLAG_NONE=0,          // none
      CONFIRM_FLAG_TICK=1,          // add quoted price into MT5 price stream
      //--- enumeration borders
      CONFIRM_FLAG_ALL =CONFIRM_FLAG_TICK
     };
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTConfirm* confirm)=0;
   virtual MTAPIRES  Clear(void)=0;
   virtual LPCWSTR   Print(MTAPISTR& string) const=0;
   //--- request id
   virtual UINT      ID(void) const=0;
   virtual MTAPIRES  ID(const UINT id)=0;
   //--- confirmation retcode
   virtual MTAPIRES  Retcode(void) const=0;
   virtual MTAPIRES  Retcode(const MTAPIRES retcode)=0;
   //--- confirmed volume
   virtual UINT64    Volume(void) const=0;
   virtual MTAPIRES  Volume(const UINT64 volume)=0;
   //--- confirmed price
   virtual double    Price(void) const=0;
   virtual MTAPIRES  Price(const double price)=0;
   //--- confirmed tick bid
   virtual double    TickBid(void) const=0;
   virtual MTAPIRES  TickBid(const double tickbid)=0;
   //--- confirmed tick ask
   virtual double    TickAsk(void) const=0;
   virtual MTAPIRES  TickAsk(const double tickask)=0;
   //--- confirmed tick last
   virtual double    TickLast(void) const=0;
   virtual MTAPIRES  TickLast(const double ticklast)=0;
   //--- confirmation comment
   virtual LPCWSTR   Comment(void) const=0;
   virtual MTAPIRES  Comment(LPCWSTR comment)=0;
   //--- flags
   virtual UINT      Flags(void) const=0;
   virtual MTAPIRES  Flags(const UINT flags)=0;
   //--- linked deal id in external system
   virtual LPCWSTR   DealID(void) const=0;
   virtual MTAPIRES  DealID(LPCWSTR deal_id)=0;
   //--- linked order id in external system
   virtual LPCWSTR   OrderID(void) const=0;
   virtual MTAPIRES  OrderID(LPCWSTR order_id)=0;
   //--- execution price on gateway
   virtual double    PriceGateway(void) const=0;
   virtual MTAPIRES  PriceGateway(const double price)=0;
   //--- linked  position ticket in external system
   virtual LPCWSTR   PositionExternalID(void) const=0;
   virtual MTAPIRES  PositionExternalID(LPCWSTR id)=0;
   //--- external trade system return code
   virtual int       ExternalRetcode(void) const=0;
   virtual MTAPIRES  ExternalRetcode(const int retcode)=0;
   //--- confirmed volume with extended accuracy
   virtual UINT64    VolumeExt(void) const=0;
   virtual MTAPIRES  VolumeExt(const UINT64 volume)=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTConfirm(void) {}
  };

class IMTExecution
  {
public:
   //--- execution state
   enum EnTradeExecutions
     {
      TE_ORDER_FIRST               =0,
      //--- order filling
      TE_ORDER_NEW_REQUEST         =0,
      TE_ORDER_NEW                 =1,
      TE_ORDER_FILL                =2,
      TE_ORDER_REJECT              =3,
      //--- order modify
      TE_ORDER_MODIFY_REQUEST      =4,
      TE_ORDER_MODIFY              =5,
      TE_ORDER_MODIFY_REJECT       =6,
      //--- order cancel
      TE_ORDER_CANCEL_REQUEST      =7,
      TE_ORDER_CANCEL              =8,
      TE_ORDER_CANCEL_REJECT       =9,
      //--- order others
      TE_ORDER_CHANGE_ID           =10,
      TE_ORDER_CLOSE_BY            =11,
      TE_ORDER_EXPIRE              =12,
      TE_ORDER_LAST                =TE_ORDER_EXPIRE,
      //--- deal modifications
      TE_DEAL_FIRST                =50,
      TE_DEAL_CANCEL               =50,
      TE_DEAL_CORRECT              =51,
      TE_DEAL_EXTERNAL             =52,
      TE_DEAL_REPO                 =53,
      TE_DEAL_COMMISSION           =54,
      TE_DEAL_LAST                 =TE_DEAL_COMMISSION,
      //--- end of session executions
      TE_EOS_FIRST                 =100,
      TE_EOS_CANCEL_DAILY_ORDERS   =100,
      TE_EOS_VARIATION_MARGIN      =101,
      TE_EOS_RECALC_DEALS          =102,
      TE_EOS_SETTLEMENT            =103,
      TE_EOS_TRANSFER              =104,
      TE_EOS_CANCEL_ALL_ORDERS     =105,
      TE_EOS_ROLLOVER              =106,
      TE_EOS_LAST                  =TE_EOS_ROLLOVER,
      //--- balance operations
      TE_BALANCE_FIRST             =150,
      TE_BALANCE_CHANGE            =150,
      TE_BALANCE_CORRECT           =151,
      TE_BALANCE_UNBLOCK_PROFIT    =152,
      TE_BALANCE_LAST              =TE_BALANCE_UNBLOCK_PROFIT,
      //--- position modifications
      TE_POSITION_FIRST            =200,
      TE_POSITION_CHANGE_ID        =201,
      TE_POSITION_LAST             =TE_POSITION_CHANGE_ID,
      //--- enumeration borders
      TE_FIRST                     =TE_ORDER_FIRST,
      TE_LAST                      =TE_POSITION_LAST
     };
   //--- execution flags
   enum EnFlags
     {
      TE_FLAG_NONE                 =0x00000000,  // none
      TE_FLAG_BROADCAST            =0x00000001,  // broadcast execution will be send to all trade servers
      TE_FLAG_MARKUP               =0x00000002,  // gateway settings request to markup prices
      TE_FLAG_SETTLEMENT_COMMISSION=0x00000004,  // charge commissions on setllement - TE_EOS_SETTLEMENT
      //--- enumeration borders
      TE_FLAG_ALL                  =TE_FLAG_BROADCAST|TE_FLAG_MARKUP|TE_FLAG_SETTLEMENT_COMMISSION
     };
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTExecution* exec)=0;
   virtual MTAPIRES  Clear(void)=0;
   virtual LPCWSTR   Print(MTAPISTR& string) const=0;
   //--- id
   virtual UINT64    ID(void) const=0;
   virtual MTAPIRES  ID(const UINT64 id)=0;
   //--- execution id in external system (exchange, ECN, etc)
   virtual LPCWSTR   ExternalID(void) const=0;
   virtual MTAPIRES  ExternalID(LPCWSTR id)=0;
   //--- execution action
   virtual UINT      Action(void) const=0;
   virtual MTAPIRES  Action(const UINT action)=0;
   //--- datetime
   virtual INT64     Datetime(void) const=0;
   virtual MTAPIRES  Datetime(const INT64 datetime)=0;
   //--- login
   virtual UINT64    Login(void) const=0;
   virtual MTAPIRES  Login(const UINT64 login)=0;
   //--- group
   virtual LPCWSTR   Group(void) const=0;
   virtual MTAPIRES  Group(LPCWSTR group)=0;
   //--- flags
   virtual UINT64    Flags(void) const=0;
   virtual MTAPIRES  Flags(const UINT64 flags)=0;
   //--- symbol
   virtual LPCWSTR   Symbol(void) const=0;
   virtual MTAPIRES  Symbol(LPCWSTR symbol)=0;
   //--- digits
   virtual UINT      Digits(void) const=0;
   //--- comment
   virtual LPCWSTR   Comment(void) const=0;
   virtual MTAPIRES  Comment(LPCWSTR comment)=0;
   //--- MT5 order ticket
   virtual UINT64    Order(void) const=0;
   virtual MTAPIRES  Order(const UINT64 order)=0;
   //--- order ticket in external system (exchange, ECN, etc)
   virtual LPCWSTR   OrderExternalID(void) const=0;
   virtual MTAPIRES  OrderExternalID(LPCWSTR id)=0;
   //--- order type IMTOrder::EnOrderType
   virtual UINT      OrderType(void) const=0;
   virtual MTAPIRES  OrderType(const UINT type)=0;
   //--- order volume
   virtual UINT64    OrderVolume(void) const=0;
   virtual MTAPIRES  OrderVolume(const UINT64 volume)=0;
   //--- order price
   virtual double    OrderPrice(void) const=0;
   virtual MTAPIRES  OrderPrice(const double price)=0;
   //--- order activation flags
   virtual UINT      OrderActivationFlags(void) const=0;
   virtual MTAPIRES  OrderActivationFlags(const UINT activation)=0;
   //--- deal ticket in external system (exchange, ECN, etc)
   virtual LPCWSTR   DealExternalID(void) const=0;
   virtual MTAPIRES  DealExternalID(LPCWSTR id)=0;
   //--- deal action IMTDeal::EnDealAction
   virtual UINT      DealAction(void) const=0;
   virtual MTAPIRES  DealAction(const UINT action)=0;
   //--- deal volume
   virtual UINT64    DealVolume(void) const=0;
   virtual MTAPIRES  DealVolume(const UINT64 volume)=0;
   //--- unfilled order volume
   virtual UINT64    DealVolumeRemaind(void) const=0;
   virtual MTAPIRES  DealVolumeRemaind(const UINT64 volume)=0;
   //--- deal price
   virtual double    DealPrice(void) const=0;
   virtual MTAPIRES  DealPrice(const double price)=0;
   //--- digits set
   virtual MTAPIRES  Digits(const UINT digits)=0;
   //--- external system trade account (exchange, ECN, etc)
   virtual LPCWSTR   ExternalAccount(void) const=0;
   virtual MTAPIRES  ExternalAccount(LPCWSTR account)=0;
   //--- order trigger price
   virtual double    OrderPriceTrigger(void) const=0;
   virtual MTAPIRES  OrderPriceTrigger(const double price)=0;
   //--- EnOrderTime
   virtual UINT      OrderTypeTime(void) const=0;
   virtual MTAPIRES  OrderTypeTime(const UINT type)=0;
   //--- order expiration
   virtual INT64     OrderTimeExpiration(void) const=0;
   virtual MTAPIRES  OrderTimeExpiration(const INT64 time)=0;
   //--- EnOrderFilling
   virtual UINT      OrderTypeFill(void) const=0;
   virtual MTAPIRES  OrderTypeFill(const UINT type)=0;
   //--- session start datetime
   virtual INT64     EOSSessionStart(void) const=0;
   virtual MTAPIRES  EOSSessionStart(const INT64 start)=0;
   //--- session end datetime
   virtual INT64     EOSSessionEnd(void) const=0;
   virtual MTAPIRES  EOSSessionEnd(const INT64 end)=0;
   //--- session settlement price
   virtual double    EOSPriceSettlement(void) const=0;
   virtual MTAPIRES  EOSPriceSettlement(const double price)=0;
   //--- profit conversion price
   virtual double    EOSProfitRateBuy(void) const=0;
   virtual double    EOSProfitRateSell(void) const=0;
   virtual MTAPIRES  EOSProfitRate(const double rate_buy,const double rate_sell)=0;
   //--- tick value
   virtual double    EOSTickValue(void) const=0;
   virtual MTAPIRES  EOSTickValue(const double value)=0;
   //--- order price SL
   virtual double    OrderPriceSL(void) const=0;
   virtual MTAPIRES  OrderPriceSL(const double price)=0;
   //--- order price TP
   virtual double    OrderPriceTP(void) const=0;
   virtual MTAPIRES  OrderPriceTP(const double price)=0;
   //--- execution price on gateway
   virtual double    PriceGateway(void) const=0;
   virtual MTAPIRES  PriceGateway(const double price)=0;
   //--- order activation flags
   virtual UINT      OrderActivationMode(void) const=0;
   virtual MTAPIRES  OrderActivationMode(const UINT activation)=0;
   //--- deal commission
   virtual double    DealCommission(void) const=0;
   virtual MTAPIRES  DealCommission(const double comm)=0;
   //--- datetime in msc since 1970.01.01
   virtual INT64     DatetimeMsc(void) const=0;
   virtual MTAPIRES  DatetimeMsc(const INT64 datetime)=0;
   //--- symbol field new value
   virtual LPCWSTR   SymbolNew(void) const=0;
   virtual MTAPIRES  SymbolNew(LPCWSTR symbol)=0;
   //--- internal data for API usage
   virtual MTAPIRES  ApiDataUpdate(const UINT pos,const USHORT app_id,const UCHAR id,const INT64 value)=0;
   virtual MTAPIRES  ApiDataUpdate(const UINT pos,const USHORT app_id,const UCHAR id,const UINT64 value)=0;
   virtual MTAPIRES  ApiDataUpdate(const UINT pos,const USHORT app_id,const UCHAR id,const double value)=0;
   virtual MTAPIRES  ApiDataNext(const UINT pos,USHORT& app_id,UCHAR& id,INT64& value) const=0;
   virtual MTAPIRES  ApiDataNext(const UINT pos,USHORT& app_id,UCHAR& id,UINT64& value) const=0;
   virtual MTAPIRES  ApiDataNext(const UINT pos,USHORT& app_id,UCHAR& id,double& value) const=0;
   //--- deal storage
   virtual double    DealStorage(void) const=0;
   virtual MTAPIRES  DealStorage(const double storage)=0;
   //--- rollover values
   virtual double    EOSRolloverValueLong(void) const=0;
   virtual double    EOSRolloverValueShort(void) const=0;
   virtual MTAPIRES  EOSRolloverValue(const double value_long,const double value_short)=0;
   //--- deal reason
   virtual UINT      DealReason(void) const=0;
   virtual MTAPIRES  DealReason(const UINT reason)=0;
   //--- gateway id
   virtual UINT64    GatewayID(void) const=0;
   virtual MTAPIRES  GatewayID(const UINT64 gateway_id)=0;
   //--- MT5 position ticket
   virtual UINT64    Position(void) const=0;
   virtual MTAPIRES  Position(const UINT64 position)=0;
   //--- MT5 position ticket
   virtual UINT64    PositionBy(void) const=0;
   virtual MTAPIRES  PositionBy(const UINT64 position)=0;
   //--- position ticket in external system (exchange, ECN, etc)
   virtual LPCWSTR   PositionExternalID(void) const=0;
   virtual MTAPIRES  PositionExternalID(LPCWSTR id)=0;
   //--- position ticket in external system (exchange, ECN, etc)
   virtual LPCWSTR   PositionByExternalID(void) const=0;
   virtual MTAPIRES  PositionByExternalID(LPCWSTR id)=0;
   //--- external trade system return code
   virtual int       ExternalRetcode(void) const=0;
   virtual MTAPIRES  ExternalRetcode(const int retcode)=0;
   //--- order volume with extendede accuracy
   virtual UINT64    OrderVolumeExt(void) const=0;
   virtual MTAPIRES  OrderVolumeExt(const UINT64 volume)=0;
   //--- deal volume with extendede accuracy
   virtual UINT64    DealVolumeExt(void) const=0;
   virtual MTAPIRES  DealVolumeExt(const UINT64 volume)=0;
   //--- unfilled order volume with extendede accuracy
   virtual UINT64    DealVolumeRemaindExt(void) const=0;
   virtual MTAPIRES  DealVolumeRemaindExt(const UINT64 volume)=0;
   //--- internal data for API usage
   virtual MTAPIRES  ApiDataSet(const USHORT app_id,const UCHAR id,const INT64 value)=0;
   virtual MTAPIRES  ApiDataSet(const USHORT app_id,const UCHAR id,const UINT64 value)=0;
   virtual MTAPIRES  ApiDataSet(const USHORT app_id,const UCHAR id,const double value)=0;
   virtual MTAPIRES  ApiDataGet(const USHORT app_id,const UCHAR id,INT64& value) const=0;
   virtual MTAPIRES  ApiDataGet(const USHORT app_id,const UCHAR id,UINT64& value) const=0;
   virtual MTAPIRES  ApiDataGet(const USHORT app_id,const UCHAR id,double& value) const=0;
   virtual MTAPIRES  ApiDataRawSet(const void *data,const UINT datalen)=0;
   virtual LPCVOID   ApiDataRawGet(void) const=0;
   virtual UINT      ApiDataRawMax(void) const=0;
   virtual MTAPIRES  ApiDataClear(const USHORT app_id)=0;
   virtual MTAPIRES  ApiDataClearAll(void)=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTExecution(void) {}
  };

class IMTTradeSink
  {
public:
   //--- trade request event
   virtual void      OnTradeRequestAdd(const IMTRequest*   /*request*/,
                                       const IMTConGroup*  /*group*/,
                                       const IMTConSymbol* /*symbol*/,
                                       const IMTPosition*  /*position*/,
                                       const IMTOrder*     /*order*/)    {  }
   //--- trade request process event
   virtual void      OnTradeRequestUpdate(const IMTRequest* /*request*/) {  }
   //--- trade request process event
   virtual void      OnTradeRequestDelete(const IMTRequest* /*request*/) {  }
   //--- trade request process event
   virtual void      OnTradeRequestProcess(const IMTRequest*   /*request*/,
                                           const IMTConfirm*   /*confirm*/,
                                           const IMTConGroup*  /*group*/,
                                           const IMTConSymbol* /*symbol*/,
                                           const IMTPosition*  /*position*/,
                                           const IMTOrder*     /*order*/,
                                           const IMTDeal*      /*deal*/) {   }
   //--- trade request add hook
   virtual MTAPIRES  HookTradeRequestAdd(IMTRequest*         /*request*/,
                                         const IMTConGroup*  /*group*/,
                                         const IMTConSymbol* /*symbol*/,
                                         const IMTPosition*  /*position*/,
                                         const IMTOrder*     /*order*/,
                                         IMTOrder*           /*order_new*/) { return(MT_RET_OK); }
   //--- trade request route hook
   virtual MTAPIRES  HookTradeRequestRoute(IMTRequest*         /*request*/,
                                           IMTConfirm*         /*confirm*/,
                                           const IMTConGroup*  /*group*/,
                                           const IMTConSymbol* /*symbol*/,
                                           const IMTPosition*  /*position*/,
                                           const IMTOrder*     /*order*/)   { return(MT_RET_OK); }
   //--- trade request deal hook
   virtual MTAPIRES  HookTradeRequestProcess(const IMTRequest*   /*request*/,
                                             const IMTConfirm*   /*confirm*/,
                                             const IMTConGroup*  /*group*/,
                                             const IMTConSymbol* /*symbol*/,
                                             IMTPosition*        /*position*/,
                                             IMTOrder*           /*order*/,
                                             IMTDeal*            /*deal*/)  { return(MT_RET_OK); }
   //--- rollover calculation hook
   virtual MTAPIRES HookTradeRollover(const INT64         /*datetime*/,
                                      const IMTConGroup*  /*group*/,
                                      const IMTConSymbol* /*symbol*/,
                                      const IMTPosition*  /*position*/,
                                      const double        /*original_value*/,
                                      double&             /*new_value*/)    { return(MT_RET_OK); }
   //--- interest calculation hook
   virtual MTAPIRES HookTradeInterest(const INT64         /*datetime*/,
                                      const IMTConGroup*  /*group*/,
                                      const IMTAccount*   /*account*/,
                                      const double        /*original_value*/,
                                      double&             /*new_value*/)    { return(MT_RET_OK); }
   //--- interest charge hook
   virtual MTAPIRES HookTradeInterestCharge(const INT64         /*datetime*/,
                                            const IMTConGroup*  /*group*/,
                                            const IMTUser*      /*user*/,
                                            const double        /*original_value*/,
                                            double&             /*new_value*/)    { return(MT_RET_OK); }
   //--- order commission calculation
   virtual MTAPIRES HookTradeCommissionOrder(const IMTConCommission*  /*commission*/,
                                             const IMTConGroup*       /*group*/,
                                             const IMTConSymbol*      /*symbol*/,
                                             const IMTOrder*          /*order*/,
                                             const double             /*original_value*/,
                                             double&                  /*new_value*/)  { return(MT_RET_OK); }
   //--- final commission calculation
   virtual MTAPIRES HookTradeCommissionCharge(const INT64             /*period_start*/,
                                              const INT64             /*period_end*/,
                                              const IMTConCommission* /*commission*/,
                                              const IMTConGroup*      /*group*/,
                                              const IMTUser*          /*user*/,
                                              const double            /*original_value*/,
                                              double&                 /*new_value*/)  { return(MT_RET_OK); }
   //--- order commission calculation
   virtual MTAPIRES HookTradeCommissionDeal(const IMTConCommission*   /*commission*/,
                                             const IMTConGroup*       /*group*/,
                                             const IMTConSymbol*      /*symbol*/,
                                             const IMTDeal*           /*deal*/,
                                             const double             /*original_value*/,
                                             double&                  /*new_value*/)  { return(MT_RET_OK); }
   //--- trade execution event
   virtual void      OnTradeExecution(const IMTConGateway* /*gateway*/,
                                      const IMTExecution*  /*execution*/,
                                      const IMTConGroup*   /*group*/,
                                      const IMTConSymbol*  /*symbol*/,
                                      const IMTPosition*   /*position*/,
                                      const IMTOrder*      /*order*/,
                                      const IMTDeal*       /*deal*/) {   }
   //--- trade execution hook
   virtual MTAPIRES  HookTradeExecution(const IMTConGateway* /*gateway*/,
                                        const IMTExecution*  /*execution*/,
                                        const IMTConGroup*   /*group*/,
                                        const IMTConSymbol*  /*symbol*/,
                                        IMTPosition*         /*position*/,
                                        IMTOrder*            /*order*/,
                                        IMTDeal*             /*deal*/)  { return(MT_RET_OK); }
   //--- trade request refused on pre-trade control event
   virtual void      OnTradeRequestRefuse(const IMTRequest* /*request*/) {  }
   //--- trade request process event
   virtual void      OnTradeRequestProcessCloseBy(const IMTRequest*   /*request*/,
                                           const IMTConfirm*   /*confirm*/,
                                           const IMTConGroup*  /*group*/,
                                           const IMTConSymbol* /*symbol*/,
                                           const IMTPosition*  /*position*/,
                                           const IMTOrder*     /*order*/,
                                           const IMTDeal*      /*deal*/,
                                           const IMTDeal*      /*deal_by*/) {   }
   //--- trade request deal hook
   virtual MTAPIRES  HookTradeRequestProcessCloseBy(const IMTRequest*   /*request*/,
                                                    const IMTConfirm*   /*confirm*/,
                                                    const IMTConGroup*  /*group*/,
                                                    const IMTConSymbol* /*symbol*/,
                                                    IMTPosition*        /*position*/,
                                                    IMTOrder*           /*order*/,
                                                    IMTDeal*            /*deal*/,
                                                    IMTDeal*            /*deal_by*/) { return(MT_RET_OK); }
   //--- interest charge hook
   virtual MTAPIRES HookTradeInterestChargeDeal(const INT64         /*datetime*/,
                                                const IMTConGroup*  /*group*/,
                                                const IMTUser*      /*user*/,
                                                IMTDeal*            /*deal*/)    { return(MT_RET_OK); }
   //--- trade request route hook
   virtual MTAPIRES  HookTradeRequestRuleFilter(IMTRequest*         /*request*/,
                                                IMTConRoute*        /*rule*/,
                                                const IMTConGroup*  /*group*/)   { return(MT_RET_OK_NONE); }
   //--- trade request route hook
   virtual MTAPIRES  HookTradeRequestRuleApply(IMTRequest*         /*request*/,
                                               IMTConRoute*        /*rule*/,
                                               const IMTConGroup*  /*group*/)    { return(MT_RET_OK_NONE); }

   //--- trade execution event for close by
   virtual void      OnTradeExecutionCloseBy(const IMTExecution* /*execution*/,
                                             const IMTConGroup*  /*group*/,
                                             const IMTConSymbol* /*symbol*/,
                                             const IMTPosition*  /*position*/,
                                             const IMTOrder*     /*order*/,
                                             const IMTDeal*      /*deal*/,
                                             const IMTDeal*      /*deal_by*/) {   }
   //--- trade execution hook for close by
   virtual MTAPIRES  HookTradeExecutionCloseBy(const IMTExecution* /*execution*/,
                                               const IMTConGroup*  /*group*/,
                                               const IMTConSymbol* /*symbol*/,
                                               IMTPosition*        /*position*/,
                                               IMTOrder*           /*order*/,
                                               IMTDeal*            /*deal*/,
                                               IMTDeal*            /*deal_by*/)      { return(MT_RET_OK); }
   //--- position split event
   virtual void      OnTradeSplit(const UINT /*shares_new*/,const UINT /*shares_old*/,
                                  const UINT /*round_prices*/,const UINT /*round_volumes*/,const UINT /*flags*/,
                                  const double /*adjustment*/,
                                  const IMTConGroup* /*group*/,const IMTConSymbol* /*symbol*/,
                                  const IMTPosition* /*position_old*/,const IMTPosition* /*position_new*/) {   }
   virtual MTAPIRES  HookTradeSplit(const UINT /*shares_new*/,const UINT /*shares_old*/,
                                    const UINT /*round_prices*/,const UINT /*round_volumes*/,const UINT /*flags*/,
                                    const double /*adjustment*/,
                                    const IMTConGroup* /*group*/,const IMTConSymbol* /*symbol*/,
                                    IMTPosition* /*position_old*/,IMTPosition* /*position_new*/) { return(MT_RET_OK); }
  };

class IMTEndOfDaySink
  {
public:
   //--- end of day events
   virtual void      OnEODStart(const INT64 /*datetime*/,const INT64 /*prev_datetime*/)                                        { }
   virtual void      OnEODGroupStart(const INT64 /*datetime*/,const INT64 /*prev_datetime*/,const IMTConGroup* /*group*/)      { }
   virtual void      OnEODGroupCommissions(const INT64 /*datetime*/,const INT64 /*prev_datetime*/,const IMTConGroup* /*group*/){ }
   virtual void      OnEODGroupInterest(const INT64 /*datetime*/,const INT64 /*prev_datetime*/,const IMTConGroup* /*group*/)   { }
   virtual void      OnEODGroupStatements(const INT64 /*datetime*/,const INT64 /*prev_datetime*/,const IMTConGroup* /*group*/) { }
   virtual void      OnEODGroupRollovers(const INT64 /*datetime*/,const INT64 /*prev_datetime*/,const IMTConGroup* /*group*/)  { }
   virtual void      OnEODGroupFinish(const INT64 /*datetime*/,const INT64 /*prev_datetime*/,const IMTConGroup* /*group*/)     { }
   virtual void      OnEODFinish(const INT64 /*datetime*/,const INT64 /*prev_datetime*/)                                       { }
   //--- end of month events
   virtual void      OnEOMStart(const INT64 /*datetime*/,const INT64 /*prev_datetime*/)                                        { }
   virtual void      OnEOMGroupStart(const INT64 /*datetime*/,const INT64 /*prev_datetime*/,const IMTConGroup* /*group*/)      { }
   virtual void      OnEOMGroupCommissions(const INT64 /*datetime*/,const INT64 /*prev_datetime*/,const IMTConGroup* /*group*/){ }
   virtual void      OnEOMGroupInterest(const INT64 /*datetime*/,const INT64 /*prev_datetime*/,const IMTConGroup* /*group*/)   { }
   virtual void      OnEOMGroupStatements(const INT64 /*datetime*/,const INT64 /*prev_datetime*/,const IMTConGroup* /*group*/) { }
   virtual void      OnEOMGroupFinish(const INT64 /*datetime*/,const INT64 /*prev_datetime*/,const IMTConGroup* /*group*/)     { }
   virtual void      OnEOMFinish(const INT64 /*datetime*/,const INT64 /*prev_datetime*/)                                       { }
  };

class IMTRequestArray
  {
public:
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTRequestArray* array)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- add
   virtual MTAPIRES  Add(IMTRequest* request)=0;
   virtual MTAPIRES  AddCopy(const IMTRequest* request)=0;
   virtual MTAPIRES  Add(IMTRequestArray* array)=0;
   virtual MTAPIRES  AddCopy(const IMTRequestArray* array)=0;
   //--- delete & detach
   virtual MTAPIRES  Delete(const UINT pos)=0;
   virtual IMTRequest* Detach(const UINT pos)=0;
   //--- update
   virtual MTAPIRES  Update(const UINT pos,IMTRequest* request)=0;
   virtual MTAPIRES  UpdateCopy(const UINT pos,const IMTRequest* request)=0;
   virtual MTAPIRES  Shift(const UINT pos,const int shift)=0;
   //--- data access
   virtual UINT        Total(void) const=0;
   virtual IMTRequest* Next(const UINT index) const=0;
   //--- sorting and search
   virtual MTAPIRES  Sort(MTSortFunctionPtr sort_function)=0;
   virtual int       Search(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchGreatOrEq(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchGreater(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchLessOrEq(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchLess(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchLeft(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchRight(const void *key,MTSortFunctionPtr sort_function) const=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTRequestArray(void) {}
  };

#pragma pack(push,1)
struct MTBookItem
  {
   //--- book item type
   enum EnBookItem
     {
      ItemReset      =0,                                    // reset book item
      ItemSell       =1,                                    // sell item
      ItemBuy        =2,                                    // buy item
      ItemSellMarket =3,                                    // sell item by market
      ItemBuyMarket  =4                                     // buy item by market
     };
   UINT              type;                                  // EnBookItem
   double            price;                                 // deal price
   INT64             volume;                                // deal volume - only integer values
   INT64             volume_ext;                            // deal volume with extended accuracy - 8 digits
   UINT              reserved[6];                           // reserved
  };
#pragma pack(pop)

#pragma pack(push,1)
struct MTBook
  {
   //--- book flags
   enum EnBookFlags
     {
      FLAG_PRE_AUCTION=1,                                   // pre-auction book state
      FLAG_SNAPSHOT   =2,                                   // snapshot of book
      //--- enumeration borders
      FLAG_NONE       =0,                                   // none
      FLAG_ALL        =FLAG_PRE_AUCTION|FLAG_SNAPSHOT       // all flags
     };
   wchar_t           symbol[32];                            // symbol
   MTBookItem        items[32*4];                           // book transactions
   UINT              items_total;                           // book transactions count
   UINT64            flags;                                 // flags
   INT64             datetime;                              // datetime
   INT64             datetime_msc;                          // datetime
   UINT              reserved[58];                          // reserved
  };
#pragma pack(pop)

typedef MTBook MTBookDiff;

class IMTBookSink
  {
public:
   virtual void      OnBook(const MTBook& /*book*/) {}
  };

enum EnSplitRounding
  {
   SPLIT_ROUNDING_STANDARD=0,  // standard mathematical rounding
   SPLIT_ROUNDING_DOWN    =1,  // round half up
   SPLIT_ROUNDING_UP      =2,  // round half down
//--- enumeration borders
   SPLIT_ROUNDING_FIRST   =SPLIT_ROUNDING_STANDARD,
   SPLIT_ROUNDING_LAST    =SPLIT_ROUNDING_UP
  };

#pragma pack(push,1)
struct MTChartBar
  {
   INT64             datetime;          // datetime
   //--- prices
   double            open;              // open price
   double            high;              // high price
   double            low;               // low price
   double            close;             // close price
   //--- volumes
   UINT64            tick_volume;       // tick volume
   INT32             spread;            // spread
   UINT64            volume;            // volume
  };
#pragma pack(pop)

class IMTCertificate
  {
public:
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTCertificate *certificate)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- certificate open/save/close/raw access
   virtual MTAPIRES  Open(LPCWSTR filename)=0;
   virtual MTAPIRES  OpenMemory(const void *data,const UINT size)=0;
   virtual MTAPIRES  Save(LPCWSTR filename) const=0;
   virtual MTAPIRES  Close(void)=0;
   virtual LPCVOID   Raw(void) const=0;
   virtual UINT      RawSize(void) const=0;
   virtual MTAPIRES  CommonReserved1(void)=0;
   virtual MTAPIRES  CommonReserved2(void)=0;
   virtual MTAPIRES  CommonReserved3(void)=0;
   virtual MTAPIRES  CommonReserved4(void)=0;
   virtual MTAPIRES  CommonReserved5(void)=0;
   //--- certificate properties
   virtual bool      IsOpened(void) const=0;
   virtual bool      IsRoot(void) const=0;
   virtual bool      IsCA(void) const=0;
   virtual bool      IsEqual(const IMTCertificate *certificate)=0;
   virtual bool      IsReserved1(void)=0;
   virtual bool      IsReserved2(void)=0;
   virtual bool      IsReserved3(void)=0;
   virtual bool      IsReserved4(void)=0;
   virtual bool      IsReserved5(void)=0;
   //--- certificate number and fields
   virtual UINT64    SerialNumber(void) const=0;
   virtual INT64     ValidFrom(void) const=0;
   virtual INT64     ValidTo(void) const=0;
   virtual MTAPIRES  NameCommon(MTAPISTR& name) const=0;
   virtual MTAPIRES  NameIssuer(MTAPISTR& name) const=0;
   virtual MTAPIRES  NameOrganization(MTAPISTR& name) const=0;
   virtual MTAPIRES  NameOrganizationUnit(MTAPISTR& name) const=0;
   virtual MTAPIRES  NameGiven(MTAPISTR& name) const=0;
   virtual MTAPIRES  NameReserved1(void) const=0;
   virtual MTAPIRES  NameReserved2(void) const=0;
   virtual MTAPIRES  NameReserved3(void) const=0;
   virtual MTAPIRES  NameReserved4(void) const=0;
   virtual MTAPIRES  NameReserved5(void) const=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTCertificate(void) {}
  };

class IMTConSpreadLeg
  {
public:
   //--- leg modes
   enum EnLegMode
     {
      LEG_MODE_SYMBOL    =0, // symbol specified by Symbol
      LEG_MODE_FUTURES   =1, // symbol specified by basis in Symbol + expiration range TimeFrom-TimeTo
      //--- enumeration borders
      LEG_MODE_FIRST     =LEG_MODE_SYMBOL,
      LEG_MODE_LAST      =LEG_MODE_FUTURES
     };
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTConSpreadLeg* leg)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- mode
   virtual UINT      Mode(void) const=0;
   virtual MTAPIRES  Mode(const UINT mode)=0;
   //--- symbol
   virtual LPCWSTR   Symbol(void) const=0;
   virtual MTAPIRES  Symbol(LPCWSTR symbol)=0;
   //--- time from
   virtual INT64     TimeFrom(void) const=0;
   virtual MTAPIRES  TimeFrom(const INT64 from)=0;
   //--- time to
   virtual INT64     TimeTo(void) const=0;
   virtual MTAPIRES  TimeTo(const INT64 to)=0;
   //--- ratio
   virtual UINT64    Ratio(void) const=0;
   virtual MTAPIRES  Ratio(const UINT64 ratio)=0;
   //--- ratio double
   virtual double    RatioDbl(void) const=0;
   virtual MTAPIRES  RatioDbl(const double ratio)=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTConSpreadLeg(void) {}
  };

class IMTConSpread
  {
public:
   //--- spread types
   enum EnSpreadMarginType
     {
      MARGIN_TYPE_VALUE    =0,
      MARGIN_TYPE_MAXIMAL  =1,
      MARGIN_TYPE_CME_INTER=2,
      MARGIN_TYPE_CME_INTRA=3,
      //--- enumeration borders
      MARGIN_TYPE_FIRST    =MARGIN_TYPE_VALUE,
      MARGIN_TYPE_LAST     =MARGIN_TYPE_CME_INTRA
     };
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTConSpread* spread)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- id
   virtual UINT      ID(void) const=0;
   //--- margin type
   virtual UINT      MarginType(void) const=0;
   virtual MTAPIRES  MarginType(const UINT type)=0;
   //--- initial margin
   virtual double    MarginInitial(void) const=0;
   virtual MTAPIRES  MarginInitial(const double margin)=0;
   //--- maintenance margin
   virtual double    MarginMaintenance(void) const=0;
   virtual MTAPIRES  MarginMaintenance(const double margin)=0;
   //--- leg A
   virtual MTAPIRES  ALegAdd(IMTConSpreadLeg* leg)=0;
   virtual MTAPIRES  ALegUpdate(const UINT pos,const IMTConSpreadLeg* leg)=0;
   virtual MTAPIRES  ALegDelete(const UINT pos)=0;
   virtual MTAPIRES  ALegClear(void)=0;
   virtual MTAPIRES  ALegShift(const UINT pos,const int shift)=0;
   virtual UINT      ALegTotal(void) const=0;
   virtual MTAPIRES  ALegNext(const UINT pos,IMTConSpreadLeg* leg) const=0;
   //--- leg B
   virtual MTAPIRES  BLegAdd(IMTConSpreadLeg* leg)=0;
   virtual MTAPIRES  BLegUpdate(const UINT pos,const IMTConSpreadLeg* leg)=0;
   virtual MTAPIRES  BLegDelete(const UINT pos)=0;
   virtual MTAPIRES  BLegClear(void)=0;
   virtual MTAPIRES  BLegShift(const UINT pos,const int shift)=0;
   virtual UINT      BLegTotal(void) const=0;
   virtual MTAPIRES  BLegNext(const UINT pos,IMTConSpreadLeg* leg) const=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTConSpread(void) {}
  };

class IMTConSpreadSink
  {
public:
   virtual void      OnSpreadAdd(const IMTConSpread*    /*config*/) {  }
   virtual void      OnSpreadUpdate(const IMTConSpread* /*config*/) {  }
   virtual void      OnSpreadDelete(const IMTConSpread* /*config*/) {  }
   virtual void      OnSpreadSync(void)                             {  }
  };

class IMTOnlineArray
  {
public:
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTOnlineArray* array)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- add
   virtual MTAPIRES  Add(IMTOnline* online)=0;
   virtual MTAPIRES  AddCopy(const IMTOnline* online)=0;
   virtual MTAPIRES  Add(IMTOnlineArray* array)=0;
   virtual MTAPIRES  AddCopy(const IMTOnlineArray* array)=0;
   //--- delete & detach
   virtual MTAPIRES  Delete(const UINT pos)=0;
   virtual IMTOnline* Detach(const UINT pos)=0;
   //--- update
   virtual MTAPIRES  Update(const UINT pos,IMTOnline* online)=0;
   virtual MTAPIRES  UpdateCopy(const UINT pos,const IMTOnline* online)=0;
   virtual MTAPIRES  Shift(const UINT pos,const int shift)=0;
   //--- data access
   virtual UINT      Total(void) const=0;
   virtual IMTOnline* Next(const UINT index) const=0;
   //--- sorting and search
   virtual MTAPIRES  Sort(MTSortFunctionPtr sort_function)=0;
   virtual int       Search(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchGreatOrEq(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchGreater(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchLessOrEq(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchLess(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchLeft(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchRight(const void *key,MTSortFunctionPtr sort_function) const=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTOnlineArray(void) {}
  };

class IMTRequestSink
  {
public:
   virtual void      OnRequestAdd(const IMTRequest*    /*request*/) {  }
   virtual void      OnRequestUpdate(const IMTRequest* /*request*/) {  }
   virtual void      OnRequestDelete(const IMTRequest* /*request*/) {  }
   virtual void      OnRequestSync(void)                            {  }
  };

class IMTConEmail
  {
public:
   //--- allowed flags
   enum EnFlags
     {
      FLAG_NONE               =0, // none
      FLAG_ENABLED            =1, // mail server is enabled
      FLAG_DEFAULT            =2, // mail server is default
      //--- flags borders
      FLAG_FIRST              =FLAG_ENABLED,
      FLAG_ALL                =FLAG_ENABLED|FLAG_DEFAULT
     };
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTConEmail* email)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- name
   virtual LPCWSTR   Name(void) const=0;
   virtual MTAPIRES  Name(LPCWSTR name)=0;
   //--- sender email
   virtual LPCWSTR   SenderMail(void) const=0;
   virtual MTAPIRES  SenderMail(LPCWSTR mail)=0;
   //--- sender name
   virtual LPCWSTR   SenderName(void) const=0;
   virtual MTAPIRES  SenderName(LPCWSTR name)=0;
   //--- server
   virtual LPCWSTR   Server(void) const=0;
   virtual MTAPIRES  Server(LPCWSTR server)=0;
   //--- login
   virtual LPCWSTR   Login(void) const=0;
   virtual MTAPIRES  Login(LPCWSTR login)=0;
   //--- password
   virtual LPCWSTR   Password(void) const=0;
   virtual MTAPIRES  Password(LPCWSTR password)=0;
   //--- EnFlags
   virtual UINT64    Flags(void) const=0;
   virtual MTAPIRES  Flags(const UINT64 flags)=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTConEmail(void) {}
  };

class IMTConEmailSink
  {
public:
   virtual void      OnEmailAdd(const IMTConEmail*    /*config*/) {  }
   virtual void      OnEmailUpdate(const IMTConEmail* /*config*/) {  }
   virtual void      OnEmailDelete(const IMTConEmail* /*config*/) {  }
   virtual void      OnEmailSync(void)                            {  }
  };

class IMTConMessengerCountry
  {
public:
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTConMessengerCountry* country)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- phone code
   virtual LPCWSTR   PhoneCode(void) const=0;
   virtual MTAPIRES  PhoneCode(LPCWSTR code)=0;
   //--- message template
   virtual LPCWSTR   MessageTemplate(void) const=0;
   virtual MTAPIRES  MessageTemplate(LPCWSTR msg_template)=0;
  };

class IMTConMessengerGroup
  {
public:
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTConMessengerGroup* group)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- phone code
   virtual LPCWSTR   Group(void) const=0;
   virtual MTAPIRES  Group(LPCWSTR group)=0;
  };

class IMTConMessenger
  {
public:
   //--- allowed flags
   enum EnFlags
     {
      FLAG_NONE                  =0x00000000,  // none
      FLAG_ENABLED               =0x00000001,  // messenger is enabled
      FLAG_DEFAULT               =0x00000002,  // messenger is default
      FLAG_NOTIFY_CLIENTS        =0x00000004,  // ����� ������������ ��� ���������� ���������
      FLAG_NOTIFY_SERVICES       =0x00000008,  // ����� ������������ ��� ���������� �������� � �������������
      //--- flags borders
      FLAG_FIRST                 =FLAG_ENABLED,
      FLAG_ALL                   =FLAG_ENABLED|FLAG_DEFAULT|FLAG_NOTIFY_CLIENTS|FLAG_NOTIFY_SERVICES
     };
   //--- providers
   enum EnProviderType
     {
      //---- SMS
      PROVIDER_SMS_BULKSMS       =0,
      PROVIDER_SMS_CLICKATELL    =1,
      PROVIDER_SMS_WEBSMS        =2,
      PROVIDER_SMS_TWILIO        =3,
      PROVIDER_SMS_CMCOM         =4,
      PROVIDER_SMS_VONAGE        =5,
      PROVIDER_SMS_INFOBIP       =6,
      PROVIDER_SMS_FONIVA        =7,
      //--- SMS ranges
      PROVIDER_SMS_FIRST         =PROVIDER_SMS_BULKSMS,
      PROVIDER_SMS_LAST          =99,
      //---- Instant Messengers
      PROVIDER_IM_TELEGRAM       =100,
      PROVIDER_IM_SLACK          =101,
      PROVIDER_IM_MICROSOFT_TEAMS=102,
      //--- Instant Messengers ranges
      PROVIDER_IM_FIRST          =PROVIDER_IM_TELEGRAM,
      PROVIDER_IM_LAST           =199,
      //---- Push Services
      PROVIDER_PUSH_METAQUOTES   =200,
      PROVIDER_PUSH_UNIVERSAL    =201,
      //--- Push Services ranges
      PROVIDER_PUSH_FIRST        =PROVIDER_PUSH_METAQUOTES,
      PROVIDER_PUSH_LAST         =299,
     };
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTConMessenger* messenger)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- name
   virtual LPCWSTR   Name(void) const=0;
   virtual MTAPIRES  Name(LPCWSTR name)=0;
   //--- sender
   virtual LPCWSTR   Sender(void) const=0;
   virtual MTAPIRES  Sender(LPCWSTR sender)=0;
   //--- provider type
   virtual UINT      ProviderType(void) const=0;
   virtual MTAPIRES  ProviderType(const UINT provider)=0;
   //--- provider address
   virtual LPCWSTR   ProviderAddress(void) const=0;
   virtual MTAPIRES  ProviderAddress(LPCWSTR address)=0;
   //--- provider login
   virtual LPCWSTR   ProviderLogin(void) const=0;
   virtual MTAPIRES  ProviderLogin(LPCWSTR login)=0;
   //--- provider password
   virtual LPCWSTR   ProviderPassword(void) const=0;
   virtual MTAPIRES  ProviderPassword(LPCWSTR password)=0;
   //--- provider token
   virtual LPCWSTR   ProviderToken(void) const=0;
   virtual MTAPIRES  ProviderToken(LPCWSTR token)=0;
   //--- provider SubID
   virtual LPCWSTR   ProviderSubId(void) const=0;
   virtual MTAPIRES  ProviderSubId(LPCWSTR subid)=0;
   //--- provider currency
   virtual LPCWSTR   ProviderCurrency(void) const=0;
   virtual MTAPIRES  ProviderCurrency(LPCWSTR currency)=0;
   //--- provider currency rate
   virtual double    ProviderCurrencyRate(void) const=0;
   virtual MTAPIRES  ProviderCurrencyRate(const double rate)=0;
   //--- EnFlags
   virtual UINT64    Flags(void) const=0;
   virtual MTAPIRES  Flags(const UINT64 flags)=0;
   //--- message template
   virtual LPCWSTR   MessageTemplate(void) const=0;
   virtual MTAPIRES  MessageTemplate(LPCWSTR msg_template)=0;
   //--- country settings
   virtual MTAPIRES  CountryAdd(IMTConMessengerCountry* country)=0;
   virtual MTAPIRES  CountryUpdate(const UINT pos,const IMTConMessengerCountry* country)=0;
   virtual MTAPIRES  CountryDelete(const UINT pos)=0;
   virtual MTAPIRES  CountryClear(void)=0;
   virtual MTAPIRES  CountryShift(const UINT pos,const int shift)=0;
   virtual UINT      CountryTotal(void) const=0;
   virtual MTAPIRES  CountryNext(const UINT pos,IMTConMessengerCountry* country) const=0;
   //--- group settings
   virtual MTAPIRES  GroupAdd(IMTConMessengerGroup* group)=0;
   virtual MTAPIRES  GroupUpdate(const UINT pos,const IMTConMessengerGroup* group)=0;
   virtual MTAPIRES  GroupDelete(const UINT pos)=0;
   virtual MTAPIRES  GroupClear(void)=0;
   virtual MTAPIRES  GroupShift(const UINT pos,const int shift)=0;
   virtual UINT      GroupTotal(void) const=0;
   virtual MTAPIRES  GroupNext(const UINT pos,IMTConMessengerGroup* group) const=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTConMessenger(void) {}
  };

class IMTConMessengerSink
  {
public:
   virtual void      OnMessengerAdd(const IMTConMessenger*    /*config*/) {  }
   virtual void      OnMessengerUpdate(const IMTConMessenger* /*config*/) {  }
   virtual void      OnMessengerDelete(const IMTConMessenger* /*config*/) {  }
   virtual void      OnMessengerSync(void)                                {  }
  };

class IMTClient
  {
public:
   //--- genders
   enum EnGender
     {
      GENDER_UNSPECIFIED                  =0,            // unspecified
      GENDER_MALE                         =1,            // male
      GENDER_FEMALE                       =2,            // female
      //--- enumeration borders
      GENDER_FIRST                        =GENDER_UNSPECIFIED,
      GENDER_LAST                         =GENDER_FEMALE
     };
   //--- client types
   enum EnClientType
     {
      CLIENT_TYPE_UNDEFINED               =0,            // undefined
      CLIENT_TYPE_INDIVIDUAL              =1,            // individual person
      CLIENT_TYPE_CORPORATE               =2,            // corporate
      CLIENT_TYPE_FUND                    =3,            // fund
      //--- enumeration borders
      CLIENT_TYPE_FIRST                   =CLIENT_TYPE_UNDEFINED,
      CLIENT_TYPE_LAST                    =CLIENT_TYPE_FUND
     };
   //--- client statuses
   enum EnClientStatus
     {
      //--- demo account registered
      CLIENT_STATUS_UNREGISTERED          =0,           // anonymous client (demo account without any data)
      CLIENT_STATUS_REGISTERED            =100,         // client that opened demo account and filled contact data
      CLIENT_STATUS_NOTINTERESTED         =200,         // client that opened demo account and filled contact data, but doesn't have interest to open real account
      //--- preliminary account registered
      CLIENT_STATUS_APPLICATION_INCOMPLETED=300,        // client that filled data for real account
      CLIENT_STATUS_APPLICATION_COMPLETED =400,         // client that filled data for real account and attached all documents
      CLIENT_STATUS_APPLICATION_INFORMATION=500,        // need more information for next processing to open real account
      CLIENT_STATUS_APPLICATION_REJECTED  =600,         // client was rejected
      //--- client
      CLIENT_STATUS_APPROVED              =700,         // client with real account
      CLIENT_STATUS_FUNDED                =800,         // client that was funded his real account
      CLIENT_STATUS_ACTIVE                =900,         // client that have trading activity  for some period
      CLIENT_STATUS_INACTIVE              =1000,        // client that haven't trading activity  for some period
      CLIENT_STATUS_SUSPENDED             =1100,        // client that was suspended
      //--- closed 
      CLIENT_STATUS_CLOSED                =1200,        // closed by himself
      CLIENT_STATUS_TERMINATED            =1300,        // closed by company
      //--- enumeration borders
      CLIENT_STATUS_FIRST                 =CLIENT_STATUS_UNREGISTERED,
      CLIENT_STATUS_LAST                  =CLIENT_STATUS_TERMINATED
     };
   //--- preferred communications
   enum EnPreferredCommunication
     {
      PREFERRED_COMMUNICATION_UNDEFINED   =0,            // undefined
      PREFERRED_COMMUNICATION_EMAIL       =1,            // email
      PREFERRED_COMMUNICATION_PHONE       =2,            // phone
      PREFERRED_COMMUNICATION_PHONE_SMS   =3,            // SMS
      PREFERRED_COMMUNICATION_MESSENGER   =4,            // messenger
      //--- enumeration borders
      PREFERRED_COMMUNICATION_FIRST       =PREFERRED_COMMUNICATION_UNDEFINED,
      PREFERRED_COMMUNICATION_LAST        =PREFERRED_COMMUNICATION_MESSENGER,
     };
   //--- experiences
   enum EnTradingExperience
     {
      EXPERIENCE_LESS_1_YEAR              =0,            // less than one year
      EXPERIENCE_1_3_YEAR                 =1,            // from one to three years
      EXPERIENCE_ABOVE_3_YEAR             =2,            // above three years
      //--- enumeration borders
      EXPERIENCE_FIRST                    =EXPERIENCE_LESS_1_YEAR,
      EXPERIENCE_LAST                     =EXPERIENCE_ABOVE_3_YEAR,
     };
   //--- employments
   enum EnEmployment
     {
      EMPLOYMENT_UNEMPLOYED               =0,            // unemployed
      EMPLOYMENT_EMPLOYED                 =1,            // employed
      EMPLOYMENT_SELF_EMPLOYED            =2,            // self employed
      EMPLOYMENT_RETIRED                  =3,            // retired
      EMPLOYMENT_STUDENT                  =4,            // student
      EMPLOYMENT_OTHER                    =5,            // other
      //--- enumeration borders
      EMPLOYMENT_FIRST                    =EMPLOYMENT_UNEMPLOYED,
      EMPLOYMENT_LAST                     =EMPLOYMENT_OTHER
     };
   //--- employment industries
   enum EnEmploymentIndustry
     {
      INDUSTRY_NONE                       =0,            // none
      INDUSTRY_AGRICULTURE                =1,            // agriculture
      INDUSTRY_CONSTRUCTION               =2,            // construction
      INDUSTRY_MANAGEMENT                 =3,            // management
      INDUSTRY_COMMUNICATION              =4,            // communication
      INDUSTRY_EDUCATION                  =5,            // education
      INDUSTRY_GOVERNMENT                 =6,            // government
      INDUSTRY_HEALTHCARE                 =7,            // health care
      INDUSTRY_TOURISM                    =8,            // tourism
      INDUSTRY_IT                         =9,            // IT
      INDUSTRY_SECURITY                   =10,           // security
      INDUSTRY_MANUFACTURING              =11,           // manufacturing
      INDUSTRY_MARKETING                  =12,           // marketing
      INDUSTRY_SCIENCE                    =13,           // science
      INDUSTRY_ENGINEERING                =14,           // engineering
      INDUSTRY_TRANSPORT                  =15,           // transport
      INDUSTRY_OTHER                      =16,           // other
      //--- enumeration borders
      INDUSTRY_FIRST                      =INDUSTRY_AGRICULTURE,
      INDUSTRY_LAST                       =INDUSTRY_OTHER
     };
   //--- education levels
   enum EnEducationLevel
     {
      EDUCATION_LEVEL_NONE                =0,            // none
      EDUCATION_LEVEL_HIGH_SCHOOL         =1,            // high school
      EDUCATION_LEVEL_BACHELOR            =2,            // bachelor
      EDUCATION_LEVEL_MASTER              =3,            // master
      EDUCATION_LEVEL_PHD                 =4,            // PHD
      EDUCATION_LEVEL_OTHER               =5,            // other
      //--- enumeration borders
      EDUCATION_LEVEL_FIRST               =EDUCATION_LEVEL_NONE,
      EDUCATION_LEVEL_LAST                =EDUCATION_LEVEL_OTHER
     };
   //--- wealth sources
   enum EnWealthSource
     {
      WEALTH_SOURCE_EMPLOYMENT            =0,            // employment
      WEALTH_SOURCE_SAVINGS               =1,            // savings
      WEALTH_SOURCE_INHERITANCE           =2,            // inheritance
      WEALTH_SOURCE_OTHER                 =3,            // other
      //--- enumeration borders
      WEALTH_SOURCE_FIRST                 =WEALTH_SOURCE_EMPLOYMENT,
      WEALTH_SOURCE_LAST                  =WEALTH_SOURCE_OTHER
     };
   //--- client origins
   enum EnClientOrigin
     {
      CLIENT_ORIGIN_MANUAL                =0,           // manual created
      CLIENT_ORIGIN_DEMO                  =1,           // automatic was created from demo account
      CLIENT_ORIGIN_CONTEST               =2,           // automatic was created from contest account
      CLIENT_ORIGIN_PRELIMINARY           =3,           // automatic was created from preliminary account
      CLIENT_ORIGIN_REAL                  =4,           // automatic was created from real account
      //--- enumeration borders
      CLIENT_ORIGIN_FIRST                 =CLIENT_ORIGIN_MANUAL,
      CLIENT_ORIGIN_LAST                  =CLIENT_ORIGIN_REAL,
     };
   //--- KYC status
   enum EnKYCStatus
     {
      KYC_STATUS_UNDEFINED                =0,            // undefined
      KYC_STATUS_APPROVED                 =1,            // approved
      KYC_STATUS_DECLINED                 =2,            // declined
      //--- ������� ���������
      KYC_STATUS_FIRST                    =KYC_STATUS_UNDEFINED,
      KYC_STATUS_LAST                     =KYC_STATUS_DECLINED
     };
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTClient* client)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- record id
   virtual UINT64    RecordID(void) const=0;
   virtual MTAPIRES  RecordID(const UINT64 record_id)=0;
   //--- client type
   virtual UINT      ClientType(void) const=0;
   virtual MTAPIRES  ClientType(const UINT type)=0;
   //--- client status
   virtual UINT      ClientStatus(void) const=0;
   virtual MTAPIRES  ClientStatus(const UINT status)=0;
   //--- assigned manager
   virtual UINT64    AssignedManager(void) const=0;
   virtual MTAPIRES  AssignedManager(const UINT64 manager)=0;
   //--- compliance approved by
   virtual UINT64    ComplianceApprovedBy(void) const=0;
   virtual MTAPIRES  ComplianceApprovedBy(const UINT64 manager)=0;
   //--- compliance client category
   virtual LPCWSTR   ComplianceClientCategory(void) const=0;
   virtual MTAPIRES  ComplianceClientCategory(LPCWSTR category)=0;
   //--- compliance date approval
   virtual INT64     ComplianceDateApproval(void) const=0;
   virtual MTAPIRES  ComplianceDateApproval(const INT64 date)=0;
   //--- compliance date termination
   virtual INT64     ComplianceDateTermination(void) const=0;
   virtual MTAPIRES  ComplianceDateTermination(const INT64 date)=0;
   //--- lead campaign
   virtual LPCWSTR   LeadCampaign(void) const=0;
   virtual MTAPIRES  LeadCampaign(LPCWSTR campaign)=0;
   //--- lead source
   virtual LPCWSTR   LeadSource(void) const=0;
   virtual MTAPIRES  LeadSource(LPCWSTR source)=0;
   //--- introducer
   virtual LPCWSTR   Introducer(void) const=0;
   virtual MTAPIRES  Introducer(LPCWSTR introducer)=0;
   //--- person title
   virtual LPCWSTR   PersonTitle(void) const=0;
   virtual MTAPIRES  PersonTitle(LPCWSTR title)=0;
   //--- person name
   virtual LPCWSTR   PersonName(void) const=0;
   virtual MTAPIRES  PersonName(LPCWSTR name)=0;
   //--- person middle name
   virtual LPCWSTR   PersonMiddleName(void) const=0;
   virtual MTAPIRES  PersonMiddleName(LPCWSTR middle_name)=0;
   //--- person last name
   virtual LPCWSTR   PersonLastName(void) const=0;
   virtual MTAPIRES  PersonLastName(LPCWSTR last_name)=0;
   //--- person birth date
   virtual INT64     PersonBirthDate(void) const=0;
   virtual MTAPIRES  PersonBirthDate(const INT64 date)=0;
   //--- person citizenship
   virtual LPCWSTR   PersonCitizenship(void) const=0;
   virtual MTAPIRES  PersonCitizenship(LPCWSTR citizenship)=0;
   //--- person gender
   virtual UINT      PersonGender(void) const=0;
   virtual MTAPIRES  PersonGender(const UINT gender)=0;
   //--- person taxid
   virtual LPCWSTR   PersonTaxID(void) const=0;
   virtual MTAPIRES  PersonTaxID(LPCWSTR taxid)=0;
   //--- person document type
   virtual LPCWSTR   PersonDocumentType(void) const=0;
   virtual MTAPIRES  PersonDocumentType(LPCWSTR type)=0;
   //--- person document number
   virtual LPCWSTR   PersonDocumentNumber(void) const=0;
   virtual MTAPIRES  PersonDocumentNumber(LPCWSTR number)=0;
   //--- person document date
   virtual INT64     PersonDocumentDate(void) const=0;
   virtual MTAPIRES  PersonDocumentDate(const INT64 date)=0;
   //--- person document extra
   virtual LPCWSTR   PersonDocumentExtra(void) const=0;
   virtual MTAPIRES  PersonDocumentExtra(LPCWSTR extra)=0;
   //--- person employment
   virtual UINT      PersonEmployment(void) const=0;
   virtual MTAPIRES  PersonEmployment(const UINT employment)=0;
   //--- person industry
   virtual UINT      PersonIndustry(void) const=0;
   virtual MTAPIRES  PersonIndustry(const UINT industry)=0;
   //--- person education
   virtual UINT      PersonEducation(void) const=0;
   virtual MTAPIRES  PersonEducation(const UINT education)=0;
   //--- person wealth source
   virtual UINT      PersonWealthSource(void) const=0;
   virtual MTAPIRES  PersonWealthSource(const UINT source)=0;
   //--- person annual income
   virtual double    PersonAnnualIncome(void) const=0;
   virtual MTAPIRES  PersonAnnualIncome(const double income)=0;
   //--- person net worth
   virtual double    PersonNetWorth(void) const=0;
   virtual MTAPIRES  PersonNetWorth(const double worth)=0;
   //--- person annual deposit
   virtual double    PersonAnnualDeposit(void) const=0;
   virtual MTAPIRES  PersonAnnualDeposit(const double deposit)=0;
   //--- company name
   virtual LPCWSTR   CompanyName(void) const=0;
   virtual MTAPIRES  CompanyName(LPCWSTR name)=0;
   //--- company registration number
   virtual LPCWSTR   CompanyRegNumber(void) const=0;
   virtual MTAPIRES  CompanyRegNumber(LPCWSTR number)=0;
   //--- company registration date
   virtual INT64     CompanyRegDate(void) const=0;
   virtual MTAPIRES  CompanyRegDate(const INT64 date)=0;
   //--- company registration authority
   virtual LPCWSTR   CompanyRegAuthority(void) const=0;
   virtual MTAPIRES  CompanyRegAuthority(LPCWSTR authority)=0;
   //--- company VAT
   virtual LPCWSTR   CompanyVat(void) const=0;
   virtual MTAPIRES  CompanyVat(LPCWSTR vat)=0;
   //--- company LEI
   virtual LPCWSTR   CompanyLei(void) const=0;
   virtual MTAPIRES  CompanyLei(LPCWSTR lei)=0;
   //--- company license number
   virtual LPCWSTR   CompanyLicenseNumber(void) const=0;
   virtual MTAPIRES  CompanyLicenseNumber(LPCWSTR number)=0;
   //--- company license authority
   virtual LPCWSTR   CompanyLicenseAuthority(void) const=0;
   virtual MTAPIRES  CompanyLicenseAuthority(LPCWSTR authority)=0;
   //--- company country
   virtual LPCWSTR   CompanyCountry(void) const=0;
   virtual MTAPIRES  CompanyCountry(LPCWSTR country)=0;
   //--- company address
   virtual LPCWSTR   CompanyAddress(void) const=0;
   virtual MTAPIRES  CompanyAddress(LPCWSTR address)=0;
   //--- company website
   virtual LPCWSTR   CompanyWebsite(void) const=0;
   virtual MTAPIRES  CompanyWebsite(LPCWSTR website)=0;
   //--- contact preferred
   virtual UINT      ContactPreferred(void) const=0;
   virtual MTAPIRES  ContactPreferred(const UINT preferred)=0;
   //--- contact language
   virtual LPCWSTR   ContactLanguage(void) const=0;
   virtual MTAPIRES  ContactLanguage(LPCWSTR language)=0;
   //--- contact email
   virtual LPCWSTR   ContactEmail(void) const=0;
   virtual MTAPIRES  ContactEmail(LPCWSTR email)=0;
   //--- contact phone
   virtual LPCWSTR   ContactPhone(void) const=0;
   virtual MTAPIRES  ContactPhone(LPCWSTR phone)=0;
   //--- contact messengers
   virtual LPCWSTR   ContactMessengers(void) const=0;
   virtual MTAPIRES  ContactMessengers(LPCWSTR messengers)=0;
   //--- contact social networks
   virtual LPCWSTR   ContactSocialNetworks(void) const=0;
   virtual MTAPIRES  ContactSocialNetworks(LPCWSTR social_networks)=0;
   //--- �ontact last date
   virtual INT64     ContactLastDate(void) const=0;
   virtual MTAPIRES  ContactLastDate(const INT64 date)=0;
   //--- address country
   virtual LPCWSTR   AddressCountry(void) const=0;
   virtual MTAPIRES  AddressCountry(LPCWSTR country)=0;
   //--- address postcode
   virtual LPCWSTR   AddressPostcode(void) const=0;
   virtual MTAPIRES  AddressPostcode(LPCWSTR postcode)=0;
   //--- address street
   virtual LPCWSTR   AddressStreet(void) const=0;
   virtual MTAPIRES  AddressStreet(LPCWSTR street)=0;
   //--- address state
   virtual LPCWSTR   AddressState(void) const=0;
   virtual MTAPIRES  AddressState(LPCWSTR state)=0;
   //--- address city
   virtual LPCWSTR   AddressCity(void) const=0;
   virtual MTAPIRES  AddressCity(LPCWSTR city)=0;
   //--- experience of FX
   virtual UINT      ExperienceFX(void) const=0;
   virtual MTAPIRES  ExperienceFX(const UINT experience)=0;
   //--- experience of CFD
   virtual UINT      ExperienceCFD(void) const=0;
   virtual MTAPIRES  ExperienceCFD(const UINT experience)=0;
   //--- experience of Futures
   virtual UINT      ExperienceFutures(void) const=0;
   virtual MTAPIRES  ExperienceFutures(const UINT experience)=0;
   //--- experience of Stocks
   virtual UINT      ExperienceStocks(void) const=0;
   virtual MTAPIRES  ExperienceStocks(const UINT experience)=0;
   //--- trading group
   virtual LPCWSTR   TradingGroup(void) const=0;
   virtual MTAPIRES  TradingGroup(LPCWSTR group)=0;
   //--- client origin
   virtual UINT      ClientOrigin(void) const=0;
   virtual MTAPIRES  ClientOrigin(const UINT origin)=0;
   //--- client origin login
   virtual UINT64    ClientOriginLogin(void) const=0;
   virtual MTAPIRES  ClientOriginLogin(const UINT64 login)=0;
   //--- external ID
   virtual LPCWSTR   ClientExternalID(void) const=0;
   virtual MTAPIRES  ClientExternalID(LPCWSTR external_id)=0;
   //--- person document expiration date
   virtual INT64     PersonDocumentExpiration(void) const=0;
   virtual MTAPIRES  PersonDocumentExpiration(const INT64 date)=0;
   //--- KYC status
   virtual UINT      KYCStatus(void) const=0;
   virtual MTAPIRES  KYCStatus(const UINT kyc_status)=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTClient(void) {}
  };

class IMTClientArray
  {
public:
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTClientArray* array)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- add
   virtual MTAPIRES  Add(IMTClient* client)=0;
   virtual MTAPIRES  AddCopy(const IMTClient* client)=0;
   virtual MTAPIRES  Add(IMTClientArray* array)=0;
   virtual MTAPIRES  AddCopy(const IMTClientArray* array)=0;
   //--- delete & detach
   virtual MTAPIRES  Delete(const UINT pos)=0;
   virtual IMTClient *Detach(const UINT pos)=0;
   //--- update
   virtual MTAPIRES  Update(const UINT pos,IMTClient* client)=0;
   virtual MTAPIRES  UpdateCopy(const UINT pos,const IMTClient* client)=0;
   virtual MTAPIRES  Shift(const UINT pos,const int shift)=0;
   //--- data access
   virtual UINT      Total(void) const=0;
   virtual IMTClient*Next(const UINT index) const=0;
   //--- sorting and search
   virtual MTAPIRES  Sort(MTSortFunctionPtr sort_function)=0;
   virtual int       Search(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchGreatOrEq(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchGreater(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchLessOrEq(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchLess(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchLeft(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchRight(const void *key,MTSortFunctionPtr sort_function) const=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTClientArray(void) {}
  };

class IMTClientSink
  {
public:
   //--- events
   virtual void      OnClientAdd(const IMTClient*    /*client*/)           {  }
   virtual void      OnClientUpdate(const IMTClient* /*client*/)           {  }
   virtual void      OnClientDelete(const IMTClient* /*client*/)           {  }
  };

class IMTAttachment
  {
public:
   //--- file types
   enum EnFileType
     {
      FILE_TYPE_OTHER                       =0,          // other
      FILE_TYPE_TXT                         =1,          // txt
      FILE_TYPE_DOC                         =2,          // doc
      FILE_TYPE_PDF                         =3,          // pdf
      FILE_TYPE_JPG                         =4,          // jpg
      FILE_TYPE_PNG                         =5,          // png
      FILE_TYPE_BMP                         =6,          // bmp
      FILE_TYPE_ZIP                         =7,          // zip
      //--- enumeration borders
      FILE_TYPE_FIRST                       =FILE_TYPE_OTHER,
      FILE_TYPE_LAST                        =FILE_TYPE_ZIP,
     };
   //--- file flags
   enum EnFileFlags
     {
      FILE_FLAG_EMBEDDED                    =0x1,        // embedded for content (images in HTML)
      //--- enumeration borders
      FILE_FLAG_NONE                        =0,
      FILE_FLAG_ALL                         =FILE_FLAG_EMBEDDED,
     };
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTAttachment* file)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- record id
   virtual UINT64    RecordID(void) const=0;
   virtual MTAPIRES  RecordID(const UINT64 record_id)=0;
   //--- related client
   virtual UINT64    RelatedClient(void) const=0;
   virtual MTAPIRES  RelatedClient(const UINT64 record_id)=0;
   //--- file type
   virtual UINT      FileType(void) const=0;
   virtual MTAPIRES  FileType(const UINT type)=0;
   //--- file name
   virtual LPCWSTR   FileName(void) const=0;
   virtual MTAPIRES  FileName(LPCWSTR name)=0;
   //--- file content
   virtual const void* FileContent(UINT& content_size) const=0;
   virtual MTAPIRES  FileContent(const void* content,const UINT content_size)=0;
   //--- file size
   virtual UINT      FileSize(void)=0;
   //--- file flags
   virtual UINT      FileFlags(void) const=0;
   virtual MTAPIRES  FileFlags(const UINT flags)=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTAttachment(void) {}
  };

class IMTAttachmentArray
  {
public:
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTAttachmentArray* array)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- add
   virtual MTAPIRES  Add(IMTAttachment* attachment)=0;
   virtual MTAPIRES  AddCopy(const IMTAttachment* attachment)=0;
   virtual MTAPIRES  Add(IMTAttachmentArray* array)=0;
   virtual MTAPIRES  AddCopy(const IMTAttachmentArray* array)=0;
   //--- delete & detach
   virtual MTAPIRES  Delete(const UINT pos)=0;
   virtual IMTAttachment *Detach(const UINT pos)=0;
   //--- update
   virtual MTAPIRES  Update(const UINT pos,IMTAttachment* attachment)=0;
   virtual MTAPIRES  UpdateCopy(const UINT pos,const IMTAttachment* attachment)=0;
   virtual MTAPIRES  Shift(const UINT pos,const int shift)=0;
   //--- data access
   virtual UINT      Total(void) const=0;
   virtual IMTAttachment *Next(const UINT index) const=0;
   //--- sorting and search
   virtual MTAPIRES  Sort(MTSortFunctionPtr sort_function)=0;
   virtual int       Search(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchGreatOrEq(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchGreater(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchLessOrEq(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchLess(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchLeft(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchRight(const void *key,MTSortFunctionPtr sort_function) const=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTAttachmentArray(void) {}
  };

class IMTDocument
  {
public:
   //--- document types
   enum EnDocumentTypes
     {
      DOCUMENT_TYPE_OTHER                       =0,      // other
      //--- client
      DOCUMENT_TYPE_PERSONAL_IDENTITY           =1,      // proof of identity
      DOCUMENT_TYPE_PERSONAL_ADDRESS            =2,      // proof of address
      //--- company
      DOCUMENT_TYPE_REGISTERED_ADDRESS          =1000,   // registered address
      DOCUMENT_TYPE_DIRECTORS_PASSPORT          =1001,   // directors passport
      DOCUMENT_TYPE_CERTIFICATE_OF_INCORPORATION=1002,   // certificate of incorporation
      DOCUMENT_TYPE_CERTIFICATE_OF_DIRECTORS    =1003,   // certificate of directors
      DOCUMENT_TYPE_CERTIFICATE_OF_GOOD_STANDING=1004,   // certificate of good standing
      //--- enumeration borders
      DOCUMENT_TYPE_FIRST                       =DOCUMENT_TYPE_OTHER,
      DOCUMENT_TYPE_LAST                        =DOCUMENT_TYPE_CERTIFICATE_OF_GOOD_STANDING
     };
   //--- document subtypes
   enum EnDocumentSubtype
     {
      DOCUMENT_SUBTYPE_OTHER                    =0,      // other
      DOCUMENT_SUBTYPE_ID_CARD                  =1,      // id card
      DOCUMENT_SUBTYPE_PASSPORT                 =2,      // passport
      DOCUMENT_SUBTYPE_DRIVERS                  =3,      // driving license
      DOCUMENT_SUBTYPE_BANK_CARD                =4,      // bank card
      DOCUMENT_SUBTYPE_UTILITY_BILL             =5,      // utility bill
      DOCUMENT_SUBTYPE_BANK_STATEMENT           =6,      // bank statement
      DOCUMENT_SUBTYPE_TAX_STATEMENT            =7,      // tax statement
      DOCUMENT_SUBTYPE_SELFIE                   =8,      // selfie
      DOCUMENT_SUBTYPE_PROFILE_IMAGE            =9,      // profile image
      DOCUMENT_SUBTYPE_ID_DOC_PHOTO             =10,     // id documents photo
      DOCUMENT_SUBTYPE_AGREEMENT                =11,     // agreement of some sort, e.g. for processing personal info
      DOCUMENT_SUBTYPE_CONTRACT                 =12,     // some sort of contract
      DOCUMENT_SUBTYPE_RESIDENCE_PERMIT         =13,     // residence permit or registration document in the foreign city/country
      DOCUMENT_SUBTYPE_EMPLOYMENT_CERTIFICATE   =14,     // document from an employer, e.g. proof that a user works there
      DOCUMENT_SUBTYPE_DRIVERS_TRANSLATION      =15,     // translation of the driving license required in the target country
      DOCUMENT_SUBTYPE_INVESTOR_DOC             =16,     // document from an investor, e.g. documents which disclose assets of the investor
      DOCUMENT_SUBTYPE_VEHICLE_REG_CERTIFICATE  =17,     // certificate of vehicle registration
      DOCUMENT_SUBTYPE_INCOME_SOURCE            =18,     // proof of income
      DOCUMENT_SUBTYPE_PAYMENT_METHOD           =19,     // entity confirming payment (like bank card, crypto wallet, etc)
      //--- enumeration borders
      DOCUMENT_SUBTYPE_FIRST                    =DOCUMENT_SUBTYPE_OTHER,
      DOCUMENT_SUBTYPE_LAST                     =DOCUMENT_SUBTYPE_PAYMENT_METHOD
     };
   //--- document statuses
   enum EnDocumentStatus
     {
      DOCUMENT_STATUS_NEW                       =0,        // new
      DOCUMENT_STATUS_APPROVED                  =100,      // approved
      DOCUMENT_STATUS_REJECTED                  =200,      // rejected
      DOCUMENT_STATUS_ARCHIVED                  =300,      // archived
      DOCUMENT_STATUS_DELETED                   =400,      // deleted
      //--- enumeration borders
      DOCUMENT_STATUS_FIRST                     =DOCUMENT_STATUS_NEW,
      DOCUMENT_STATUS_LAST                      =DOCUMENT_STATUS_DELETED,
     };
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTDocument* document)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- record id
   virtual UINT64    RecordID(void) const=0;
   virtual MTAPIRES  RecordID(const UINT64 record_id)=0;
   //--- related client
   virtual UINT64    RelatedClient(void) const=0;
   virtual MTAPIRES  RelatedClient(const UINT64 record_id)=0;
   //--- approved date
   virtual INT64     ApprovedDate(void) const=0;
   virtual MTAPIRES  ApprovedDate(const INT64 date)=0;
   //--- approved by
   virtual UINT64    ApprovedBy(void) const=0;
   virtual MTAPIRES  ApprovedBy(const UINT64 manager)=0;
   //--- date issue
   virtual INT64     DateIssue(void) const=0;
   virtual MTAPIRES  DateIssue(const INT64 date)=0;
   //--- date expiration
   virtual INT64     DateExpiration(void) const=0;
   virtual MTAPIRES  DateExpiration(const INT64 date)=0;
   //--- document type
   virtual UINT      DocumentType(void) const=0;
   virtual MTAPIRES  DocumentType(const UINT type)=0;
   //--- document name
   virtual LPCWSTR   DocumentName(void) const=0;
   virtual MTAPIRES  DocumentName(LPCWSTR name)=0;
   //--- document comment
   virtual LPCWSTR   DocumentComment(void) const=0;
   virtual MTAPIRES  DocumentComment(LPCWSTR comment)=0;
   //--- document status
   virtual UINT      DocumentStatus(void) const=0;
   virtual MTAPIRES  DocumentStatus(const UINT status)=0;
   //--- attachments
   virtual MTAPIRES  AttachmentsAdd(const IMTAttachment *attachment)=0;
   virtual MTAPIRES  AttachmentsClear(void)=0;
   virtual UINT      AttachmentsTotal(void) const=0;
   virtual MTAPIRES  AttachmentsNext(const UINT pos,UINT64& attachment_id,MTAPISTR& attachment_name,UINT& attachment_size) const=0;
   //--- document subtype
   virtual UINT      DocumentSubtype(void) const=0;
   virtual MTAPIRES  DocumentSubtype(const UINT subtype)=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTDocument(void) {}
  };

class IMTDocumentArray
  {
public:
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTDocumentArray* array)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- add
   virtual MTAPIRES  Add(IMTDocument* document)=0;
   virtual MTAPIRES  AddCopy(const IMTDocument* document)=0;
   virtual MTAPIRES  Add(IMTDocumentArray* array)=0;
   virtual MTAPIRES  AddCopy(const IMTDocumentArray* array)=0;
   //--- delete & detach
   virtual MTAPIRES  Delete(const UINT pos)=0;
   virtual IMTDocument *Detach(const UINT pos)=0;
   //--- update
   virtual MTAPIRES  Update(const UINT pos,IMTDocument* document)=0;
   virtual MTAPIRES  UpdateCopy(const UINT pos,const IMTDocument* document)=0;
   virtual MTAPIRES  Shift(const UINT pos,const int shift)=0;
   //--- data access
   virtual UINT      Total(void) const=0;
   virtual IMTDocument*Next(const UINT index) const=0;
   //--- sorting and search
   virtual MTAPIRES  Sort(MTSortFunctionPtr sort_function)=0;
   virtual int       Search(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchGreatOrEq(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchGreater(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchLessOrEq(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchLess(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchLeft(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchRight(const void *key,MTSortFunctionPtr sort_function) const=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTDocumentArray(void) {}
  };

class IMTDocumentSink
  {
public:
   //--- events
   virtual void      OnDocumentAdd(const IMTDocument*    /*document*/)           {  }
   virtual void      OnDocumentUpdate(const IMTDocument* /*document*/)           {  }
   virtual void      OnDocumentDelete(const IMTDocument* /*document*/)           {  }
  };

class IMTComment
  {
public:
   //--- comment flags
   enum EnCommentFlags
     {
      COMMENT_FLAG_DELETED    =0x1,                   // deleted
      COMMENT_FLAG_IMPORTANT  =0x2,                   // important
      //--- enumeration borders
      COMMENT_FLAG_NONE       =0,
      COMMENT_FLAG_ALL        =COMMENT_FLAG_DELETED|COMMENT_FLAG_IMPORTANT,
     };
   //--- comment types
   enum EnCommentType
     {
      COMMENT_TYPE_UNDEFINED                =0,       // undefined
      COMMENT_TYPE_LOGRECORD                =1,       // log record
      COMMENT_TYPE_CALLRECORD               =2,       // call record
      COMMENT_TYPE_ROBOTRECORD              =3,       // robot record
      //--- enumeration borders
      COMMENT_TYPE_FIRST                    =COMMENT_TYPE_UNDEFINED,
      COMMENT_TYPE_LAST                     =COMMENT_TYPE_ROBOTRECORD,
     };
   //--- comment results
   enum EnCommentResult
     {
      COMMENT_RESULT_UNDEFINED              =0,       // undefined
      COMMENT_RESULT_CALL_NO_ANSWER         =1,       // call no answer
      COMMENT_RESULT_CALL_WRONG_NUMBER      =2,       // call wrong number
      COMMENT_RESULT_CALL_NOT_INTERESTED    =3,       // call not interested
      COMMENT_RESULT_CALL_SUCCESSFUL        =4,       // call successful
      //--- enumeration borders
      COMMENT_RESULT_FIRST                  =COMMENT_RESULT_UNDEFINED,
      COMMENT_RESULT_LAST                   =COMMENT_RESULT_CALL_SUCCESSFUL,
     };
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTComment* comment)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- record id
   virtual UINT64    RecordID(void) const=0;
   virtual MTAPIRES  RecordID(const UINT64 record_id)=0;
   //--- related client
   virtual UINT64    RelatedClient(void) const=0;
   virtual MTAPIRES  RelatedClient(const UINT64 record_id)=0;
   //--- related document
   virtual UINT64    RelatedDocument(void) const=0;
   virtual MTAPIRES  RelatedDocument(const UINT64 record_id)=0;
   //--- flags
   virtual UINT      Flags(void) const=0;
   virtual MTAPIRES  Flags(const UINT flags)=0;
   //--- extra
   virtual LPCWSTR   Extra(void) const=0;
   virtual MTAPIRES  Extra(LPCWSTR extra)=0;
   //--- text
   virtual LPCWSTR   Text(void) const=0;
   virtual MTAPIRES  Text(LPCWSTR text)=0;
   //--- comment type
   virtual UINT      CommentType(void) const=0;
   virtual MTAPIRES  CommentType(const UINT type)=0;
   //--- comment result
   virtual UINT      CommentResult(void) const=0;
   virtual MTAPIRES  CommentResult(const UINT result)=0;
   //--- attachments
   virtual MTAPIRES  AttachmentsAdd(const IMTAttachment *attachment)=0;
   virtual MTAPIRES  AttachmentsClear(void)=0;
   virtual UINT      AttachmentsTotal(void) const=0;
   virtual MTAPIRES  AttachmentsNext(const UINT pos,UINT64& attachment_id,MTAPISTR& attachment_name,UINT& attachment_size) const=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTComment(void) {}
  };

class IMTCommentArray
  {
public:
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTCommentArray* array)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- add
   virtual MTAPIRES  Add(IMTComment* comment)=0;
   virtual MTAPIRES  AddCopy(const IMTComment* comment)=0;
   virtual MTAPIRES  Add(IMTCommentArray* array)=0;
   virtual MTAPIRES  AddCopy(const IMTCommentArray* array)=0;
   //--- delete & detach
   virtual MTAPIRES  Delete(const UINT pos)=0;
   virtual IMTComment *Detach(const UINT pos)=0;
   //--- update
   virtual MTAPIRES  Update(const UINT pos,IMTComment* comment)=0;
   virtual MTAPIRES  UpdateCopy(const UINT pos,const IMTComment* comment)=0;
   virtual MTAPIRES  Shift(const UINT pos,const int shift)=0;
   //--- data access
   virtual UINT      Total(void) const=0;
   virtual IMTComment*Next(const UINT index) const=0;
   //--- sorting and search
   virtual MTAPIRES  Sort(MTSortFunctionPtr sort_function)=0;
   virtual int       Search(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchGreatOrEq(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchGreater(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchLessOrEq(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchLess(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchLeft(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchRight(const void *key,MTSortFunctionPtr sort_function) const=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTCommentArray(void) {}
  };

class IMTCommentSink
  {
public:
   //--- events
   virtual void      OnCommentAdd(const IMTComment*    /*comment*/)           {  }
   virtual void      OnCommentUpdate(const IMTComment* /*comment*/)           {  }
   virtual void      OnCommentDelete(const IMTComment* /*comment*/)           {  }
  };

class IMTConAutoCondition
  {
public:
   //--- condition types
   enum EnConditions
     {
      //--- time conditions
      CONDITION_DATETIME_DATETIME           =0,
      CONDITION_DATETIME_DATE               =1,
      CONDITION_DATETIME_TIME               =2,
      //--- time conditions borders
      CONDITION_DATETIME_FIRST              =0,
      CONDITION_DATETIME_LAST               =1000,
      //--- account conditions
      CONDITION_ACCOUNT_LOGIN               =1001,
      CONDITION_ACCOUNT_GROUP               =1002,
      CONDITION_ACCOUNT_COUNTRY             =1003,
      CONDITION_ACCOUNT_CITY                =1004,
      CONDITION_ACCOUNT_LANGUAGE            =1005,
      CONDITION_ACCOUNT_PHONE               =1006,
      CONDITION_ACCOUNT_EMAIL               =1007,
      CONDITION_ACCOUNT_COLOR               =1008,
      CONDITION_ACCOUNT_COMMENT             =1009,
      CONDITION_ACCOUNT_REGISTRATION        =1010,
      CONDITION_ACCOUNT_LASTTIME            =1011,
      CONDITION_ACCOUNT_LEVERAGE            =1012,
      CONDITION_ACCOUNT_BALANCE             =1013,
      CONDITION_ACCOUNT_CREDIT              =1014,
      CONDITION_ACCOUNT_POSITIONS           =1015,
      CONDITION_ACCOUNT_ORDERS              =1016,
      CONDITION_ACCOUNT_PROFIT              =1017,
      CONDITION_ACCOUNT_EQUITY              =1018,
      CONDITION_ACCOUNT_MARGIN              =1019,
      CONDITION_ACCOUNT_MARGIN_FREE         =1020,
      CONDITION_ACCOUNT_MARGIN_LEVEL        =1021,
      CONDITION_ACCOUNT_CURRENCY            =1022,
      CONDITION_ACCOUNT_REGISTRATION_ELAPSED=1023,
      CONDITION_ACCOUNT_LASTTIME_ELAPSED    =1024,
      CONDITION_ACCOUNT_LEAD_SOURCE         =1025,
      CONDITION_ACCOUNT_LEAD_CAMPAIGN       =1026,
      CONDITION_ACCOUNT_ONLINE              =1027,
      CONDITION_ACCOUNT_ACTIVITY_TRADE_ELAPSED=1028,
      CONDITION_ACCOUNT_ENABLED             =1029,
      CONDITION_ACCOUNT_TRADING_ENABLED     =1030,
      CONDITION_ACCOUNT_EXPERT_ENABLED      =1031,
      CONDITION_ACCOUNT_OWN_FUNDS_PERCENT   =1032,
      CONDITION_ACCOUNT_OWN_FUNDS_VOLUME    =1033,
      //--- account conditions borders
      CONDITION_ACCOUNT_FIRST               =1001,
      CONDITION_ACCOUNT_LAST                =2000,
      //--- monitoring conditions
      CONDITION_MONITOR_CPU                 =2001,
      CONDITION_MONITOR_CPU_PROCESS         =2002,
      CONDITION_MONITOR_CPU_PROCESS_THREADS =2003,
      CONDITION_MONITOR_MEMORY_FREE         =2004,
      CONDITION_MONITOR_MEMORY_PROCESS      =2005,
      CONDITION_MONITOR_DISK_FREE           =2006,
      CONDITION_MONITOR_DISK_SPEED_READ     =2007,
      CONDITION_MONITOR_DISK_SPEED_WRITE    =2008,
      CONDITION_MONITOR_DISK_QUEUE_LENGTH   =2009,
      CONDITION_MONITOR_NETWORK_CONNECTIONS =2010,
      CONDITION_MONITOR_NETWORK_BLOCKED     =2011,
      CONDITION_MONITOR_NETWORK_SOCKETS     =2012,
      CONDITION_MONITOR_NETWORK_TRAFFIC_IN  =2013,
      CONDITION_MONITOR_NETWORK_TRAFFIC_OUT =2014,
      CONDITION_MONITOR_NETWORK_RETRANSMIT  =2015,
      CONDITION_MONITOR_HANDLES_PROCESS     =2016,
      CONDITION_MONITOR_CPU_DPC             =2017,
      CONDITION_MONITOR_CPU_INTERRUPTS      =2018,
      //--- monitoring conditions borders
      CONDITION_MONITOR_FIRST               =2001,
      CONDITION_MONITOR_LAST                =3000,
      //--- platform server conditions borders
      CONDITION_SERVER_ID                   =3001,
      CONDITION_SERVER_TYPE                 =3002,
      CONDITION_SERVER_CONNECTED            =3003,
      //--- platform server conditions
      CONDITION_SERVER_FIRST                =3001,
      CONDITION_SERVER_LAST                 =4000,
      //--- finance operations conditions
      CONDITION_FINANCE_AMOUNT              =4001,
      CONDITION_FINANCE_CURRENCY            =4002,
      //--- finance operations borders
      CONDITION_FINANCE_FIRST               =4001,
      CONDITION_FINANCE_LAST                =5000,
      //--- prices conditions
      CONDITION_PRICES_SYMBOL               =5001,
      CONDITION_PRICES_LASTTIME             =5002,
      //--- prices conditions borders
      CONDITION_PRICES_FIRST                =5001,
      CONDITION_PRICES_LAST                 =6000,
      //--- trade position conditions
      CONDITION_POSITION_LOGIN              =6001,
      CONDITION_POSITION_TICKET             =6002,
      CONDITION_POSITION_SYMBOL             =6003,
      CONDITION_POSITION_VOLUME             =6004,
      CONDITION_POSITION_TYPE               =6005,
      CONDITION_POSITION_PRICE_OPEN         =6006,
      CONDITION_POSITION_PRICE_CURRENT      =6007,
      CONDITION_POSITION_PROFIT             =6008,
      CONDITION_POSITION_REASON             =6009,
      CONDITION_POSITION_TIME_CREATE        =6010,
      CONDITION_POSITION_TIME_UPDATE        =6011,
      CONDITION_POSITION_EXPERT             =6012,
      CONDITION_POSITION_TIME_CREATE_ELAPSED=6013,
      CONDITION_POSITION_TIME_UPDATE_ELAPSED=6014,
      //--- trade position condition borders
      CONDITION_POSITION_FIRST              =6001,
      CONDITION_POSITION_LAST               =7000,
      //--- trade deal conditions
      CONDITION_DEAL_LOGIN                  =7001,
      CONDITION_DEAL_TICKET                 =7002,
      CONDITION_DEAL_SYMBOL                 =7003,
      CONDITION_DEAL_VOLUME                 =7004,
      CONDITION_DEAL_TYPE                   =7005,
      CONDITION_DEAL_ENTRY                  =7006,
      CONDITION_DEAL_PRICE                  =7007,
      CONDITION_DEAL_PROFIT                 =7008,
      CONDITION_DEAL_REASON                 =7009,
      CONDITION_DEAL_TIME                   =7010,
      CONDITION_DEAL_EXPERT                 =7011,
      //--- trade deal condition borders
      CONDITION_DEAL_FIRST                  =7001,
      CONDITION_DEAL_LAST                   =8000,
      //--- internal message conditions
      CONDITION_MESSAGE_FROM                =8001,
      CONDITION_MESSAGE_FROM_NAME           =8002,
      CONDITION_MESSAGE_TO                  =8003,
      CONDITION_MESSAGE_TO_NAME             =8004,
      CONDITION_MESSAGE_SUBJECT             =8005,
      CONDITION_MESSAGE_BODY                =8006,
      //--- internal message conditions borders
      CONDITION_MESSAGE_FIRST               =8001,
      CONDITION_MESSAGE_LAST                =9000,
      //--- trade order condition
      CONDITION_ORDER_TICKET                =9001,
      CONDITION_ORDER_ORDER_ID              =9002,
      CONDITION_ORDER_LOGIN                 =9003,
      CONDITION_ORDER_SYMBOL                =9004,
      CONDITION_ORDER_TIME_SETUP            =9005,
      CONDITION_ORDER_TIME_SETUP_ELAPSED    =9006,
      CONDITION_ORDER_TIME_EXPIRATION       =9007,
      CONDITION_ORDER_TYPE                  =9008,
      CONDITION_ORDER_PRICE_ORDER           =9009,
      CONDITION_ORDER_PRICE_TRIGGER         =9010,
      CONDITION_ORDER_PRICE_CURRENT         =9011,
      CONDITION_ORDER_PRICE_SL              =9012,
      CONDITION_ORDER_PRICE_TP              =9013,
      CONDITION_ORDER_VOLUME_INITIAL        =9014,
      CONDITION_ORDER_VOLUME_CURRENT        =9015,
      CONDITION_ORDER_STATE                 =9016,
      CONDITION_ORDER_EXPERT                =9017,
      CONDITION_ORDER_POSITION_ID           =9018,
      CONDITION_ORDER_COMMENT               =9019,
      CONDITION_ORDER_CONTRACT_SIZE         =9020,
      CONDITION_ORDER_CURRENCY              =9021,
      //--- trade order conditions borders
      CONDITION_ORDER_FIRST                 =9001,
      CONDITION_ORDER_LAST                  =10000,
      //--- gateway condition
      CONDITION_GATEWAY_NAME                =10001,
      CONDITION_GATEWAY_ID                  =10002,
      CONDITION_GATEWAY_CONNECTED           =10003,
      //--- gateway condition borders
      CONDITION_GATEWAY_FIRST               =10001,
      CONDITION_GATEWAY_LAST                =11000,
      //--- datafeed condition
      CONDITION_DATAFEED_NAME               =11001,
      CONDITION_DATAFEED_CONNECTED          =11002,
      //--- datafeed condition borders
      CONDITION_DATAFEED_FIRST              =11001,
      CONDITION_DATAFEED_LAST               =12000,
      //--- KYC
      CONDITION_KYC_STATE_CODE              =12001,
      CONDITION_KYC_STATE_DESC              =12002,
      CONDITION_KYC_REJECT_REASON           =12003,
      //--- KYC
      CONDITION_KYC_FIRST                   =12001,
      CONDITION_KYC_LAST                    =13000,
      //--- borders
      CONDITION_FIRST                       =CONDITION_DATETIME_FIRST,
      CONDITION_LAST                        =CONDITION_KYC_LAST
     };
   //--- condition comparison method
   enum EnConditionRule
     {
      RULE_EQ                      =0,       // ==
      RULE_NOT_EQ                  =1,       // !=
      RULE_GREATER                 =2,       // >
      RULE_NOT_LESS                =3,       // >=
      RULE_LESS                    =4,       // <
      RULE_NOT_GREATER             =5,       // <=
      RULE_MATCH_MASK              =6,       // by mask
      //--- enumeration borders
      RULE_FIRST                   =RULE_EQ,
      RULE_LAST                    =RULE_MATCH_MASK
     };
   //--- condition types method
   enum EnConditionType
     {
      TYPE_NONE                    =0,
      TYPE_STRING                  =1,
      TYPE_INT                     =2,
      TYPE_UINT                    =3,
      TYPE_DOUBLE                  =4,
      TYPE_COLOR                   =5,
      TYPE_MONEY                   =6,
      TYPE_VOLUME                  =7,
      TYPE_DATETIME                =8,
      TYPE_LEVERAGE                =9,
      TYPE_BOOL                    =10,
      TYPE_TIME                    =11,
      TYPE_DATE                    =12,
      TYPE_PERCENT                 =13,
      TYPE_LANGUAGE                =14,
      TYPE_SERVER                  =15,
      TYPE_POSITION                =16,
      TYPE_REASON                  =17,
      TYPE_DEAL                    =18,
      TYPE_DEAL_ENTRY              =19,
      TYPE_ORDER                   =20,
      TYPE_ORDER_STATE             =21,
      TYPE_KYC_STATE               =22,
      //--- borders
      TYPE_FIRST                   =TYPE_NONE,
      TYPE_LAST                    =TYPE_KYC_STATE
     };
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTConAutoCondition* condition)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- condition type
   virtual UINT      Condition(void) const=0;
   virtual MTAPIRES  Condition(const UINT condition)=0;
   //--- condition rule
   virtual UINT      Rule(void) const=0;
   virtual MTAPIRES  Rule(const UINT rule)=0;
   //--- type
   virtual UINT      ValueType(void) const=0;
   //--- value int
   virtual INT64     ValueInt(void) const=0;
   virtual MTAPIRES  ValueInt(const INT64 value)=0;
   //--- value uint
   virtual UINT64    ValueUInt(void) const=0;
   virtual MTAPIRES  ValueUInt(const UINT64 value)=0;
   //--- value double
   virtual double    ValueDouble(void) const=0;
   virtual MTAPIRES  ValueDouble(const double value)=0;
   //--- value string
   virtual LPCWSTR   ValueString(void) const=0;
   virtual MTAPIRES  ValueString(LPCWSTR value)=0;
   //--- value color
   virtual COLORREF  ValueColor(void) const=0;
   virtual MTAPIRES  ValueColor(const COLORREF value)=0;
   //--- value money
   virtual double    ValueMoney(void) const=0;
   virtual MTAPIRES  ValueMoney(const double value)=0;
   //--- value volume
   virtual UINT64    ValueVolume(void) const=0;
   virtual MTAPIRES  ValueVolume(const UINT64 value)=0;
   //--- value datetime
   virtual INT64     ValueDatetime(void) const=0;
   virtual MTAPIRES  ValueDatetime(const INT64 value)=0;
   //--- value leverage
   virtual INT64     ValueLeverage(void) const=0;
   virtual MTAPIRES  ValueLeverage(const INT64 value)=0;
   //--- value bool
   virtual bool      ValueBool(void) const=0;
   virtual MTAPIRES  ValueBool(const bool value)=0;
   //--- value time
   virtual UINT      ValueTime(void) const=0;
   virtual MTAPIRES  ValueTime(const UINT value)=0;
   //--- value date
   virtual INT64     ValueDate(void) const=0;
   virtual MTAPIRES  ValueDate(const INT64 value)=0;
   //--- value percent
   virtual UINT      ValuePercent(void) const=0;
   virtual MTAPIRES  ValuePercent(const UINT value)=0;
   //--- value language
   virtual UINT      ValueLanguage(void) const=0;
   virtual MTAPIRES  ValueLanguage(const UINT value)=0;
   //--- value server
   virtual UINT64    ValueServer(void) const=0;
   virtual MTAPIRES  ValueServer(const UINT64 value)=0;
   //--- value position
   virtual UINT      ValuePositionType(void) const=0;
   virtual MTAPIRES  ValuePositionType(const UINT value)=0;
   //--- value reason
   virtual UINT      ValueReason(void) const=0;
   virtual MTAPIRES  ValueReason(const UINT value)=0;
   //--- value deal type
   virtual UINT      ValueDealType(void) const=0;
   virtual MTAPIRES  ValueDealType(const UINT value)=0;
   //--- value deal entry
   virtual UINT      ValueDealEntry(void) const=0;
   virtual MTAPIRES  ValueDealEntry(const UINT value)=0;
   //--- value order type
   virtual UINT      ValueOrderType(void) const=0;
   virtual MTAPIRES  ValueOrderType(const UINT value)=0;
   //--- value order state
   virtual UINT      ValueOrderState(void) const=0;
   virtual MTAPIRES  ValueOrderState(const UINT value)=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTConAutoCondition(void) {}
  };

class IMTConAutoParam
  {
public:
   //--- parameters
   enum EnParams
     {
      //--- 
      //--- ACTION_MESSAGE_PUSH
      //--- 
      PARAM_ACTION_MESSAGE_PUSH_TO_LOGINS        =0,
      PARAM_ACTION_MESSAGE_PUSH_TO_GROUPS        =1,
      PARAM_ACTION_MESSAGE_PUSH_TEXT             =2,
      //--- borders
      PARAM_ACTION_MESSAGE_PUSH_FIRST            =0,
      PARAM_ACTION_MESSAGE_PUSH_LAST             =50,
      //---
      //--- ACTION_MESSAGE_INTERNAL
      //---
      PARAM_ACTION_MESSAGE_INTERNAL_TO_LOGINS    =51,
      PARAM_ACTION_MESSAGE_INTERNAL_TO_GROUPS    =52,
      PARAM_ACTION_MESSAGE_INTERNAL_SUBJ         =53,
      PARAM_ACTION_MESSAGE_INTERNAL_TEXT         =54,
      PARAM_ACTION_MESSAGE_INTERNAL_FROM         =55,
      //--- borders
      PARAM_ACTION_MESSAGE_INTERNAL_FIRST        =51,
      PARAM_ACTION_MESSAGE_INTERNAL_LAST         =100,
      //---
      //--- ACTION_MESSAGE_SMS
      //---
      PARAM_ACTION_MESSAGE_SMS_TO_LOGINS         =101,
      PARAM_ACTION_MESSAGE_SMS_TO_GROUPS         =102,
      PARAM_ACTION_MESSAGE_SMS_TO_PHONES         =103,
      PARAM_ACTION_MESSAGE_SMS_TEXT              =104,
      //--- borders
      PARAM_ACTION_MESSAGE_SMS_FIRST             =101,
      PARAM_ACTION_MESSAGE_SMS_LAST              =150,
      //---
      //--- ACTION_MESSAGE_EMAIL
      //---
      PARAM_ACTION_MESSAGE_EMAIL_TO_LOGINS       =151,
      PARAM_ACTION_MESSAGE_EMAIL_TO_GROUPS       =152,
      PARAM_ACTION_MESSAGE_EMAIL_TO_EMAILS       =153,
      PARAM_ACTION_MESSAGE_EMAIL_SERVER          =154,
      PARAM_ACTION_MESSAGE_EMAIL_SUBJ            =155,
      PARAM_ACTION_MESSAGE_EMAIL_TEXT            =156,
      //--- borders
      PARAM_ACTION_MESSAGE_EMAIL_FIRST           =151,
      PARAM_ACTION_MESSAGE_EMAIL_LAST            =200,
      //---
      //--- ACTION_ACCOUNT_CHANGE_GROUP
      //---
      PARAM_ACTION_ACCOUNT_CHANGE_GROUP          =201,
      PARAM_ACTION_ACCOUNT_CHANGE_GROUP_LOGINS   =202,
      PARAM_ACTION_ACCOUNT_CHANGE_GROUP_GROUPS   =203,
      //--- borders
      PARAM_ACTION_ACCOUNT_CHANGE_GROUP_FIRST    =201,
      PARAM_ACTION_ACCOUNT_CHANGE_GROUP_LAST     =250,
      //--- 
      //--- ACTION_ACCOUNT_CHANGE_COLOR
      //--- 
      PARAM_ACTION_ACCOUNT_CHANGE_COLOR          =251,
      PARAM_ACTION_ACCOUNT_CHANGE_COLOR_LOGINS   =252,
      PARAM_ACTION_ACCOUNT_CHANGE_COLOR_GROUPS   =253,
      //--- borders
      PARAM_ACTION_ACCOUNT_CHANGE_COLOR_FIRST    =251,
      PARAM_ACTION_ACCOUNT_CHANGE_COLOR_LAST     =300,
      //--- 
      //--- ACTION_ACCOUNT_COMMENT
      //--- 
      PARAM_ACTION_ACCOUNT_COMMENT               =301,
      PARAM_ACTION_ACCOUNT_COMMENT_LOGINS        =302,
      PARAM_ACTION_ACCOUNT_COMMENT_GROUPS        =303,
      //--- borders
      PARAM_ACTION_ACCOUNT_COMMENT_FIRST         =301,
      PARAM_ACTION_ACCOUNT_COMMENT_LAST          =350,
      //--- 
      //--- ACTION_ACCOUNT_CLIENT_COMMENT
      //--- 
      PARAM_ACTION_ACCOUNT_CLIENT_COMMENT        =351,
      PARAM_ACTION_ACCOUNT_CLIENT_COMMENT_LOGINS =352,
      PARAM_ACTION_ACCOUNT_CLIENT_COMMENT_GROUPS =353,
      //--- borders
      PARAM_ACTION_CLIENT_COMMENT_FIRST          =351,
      PARAM_ACTION_CLIENT_COMMENT_LAST           =400,
      //--- 
      //--- ACTION_FINANCE_DEPOSIT
      //--- 
      PARAM_ACTION_FINANCE_DEPOSIT_AMOUNT        =401,
      PARAM_ACTION_FINANCE_DEPOSIT_COMMENT       =402,
      PARAM_ACTION_FINANCE_DEPOSIT_LOGINS        =403,
      PARAM_ACTION_FINANCE_DEPOSIT_GROUPS        =404,
      PARAM_ACTION_FINANCE_DEPOSIT_ACTION        =405,
      //--- borders
      PARAM_ACTION_FINANCE_DEPOSIT_FIRST         =401,
      PARAM_ACTION_FINANCE_DEPOSIT_LAST          =450,
      //--- 
      //--- ACTION_FINANCE_BONUS
      //--- 
      PARAM_ACTION_FINANCE_BONUS_AMOUNT          =451,
      PARAM_ACTION_FINANCE_BONUS_COMMENT         =452,
      PARAM_ACTION_FINANCE_BONUS_LOGINS          =453,
      PARAM_ACTION_FINANCE_BONUS_GROUPS          =454,
      //--- borders
      PARAM_ACTION_FINANCE_BONUS_FIRST           =451,
      PARAM_ACTION_FINANCE_BONUS_LAST            =500,
      //--- 
      //--- ACTION_FINANCE_CREDIT
      //--- 
      PARAM_ACTION_FINANCE_CREDIT_AMOUNT         =501,
      PARAM_ACTION_FINANCE_CREDIT_COMMENT        =502,
      PARAM_ACTION_FINANCE_CREDIT_LOGINS         =503,
      PARAM_ACTION_FINANCE_CREDIT_GROUPS         =504,
      //--- borders
      PARAM_ACTION_FINANCE_CREDIT_FIRST          =501,
      PARAM_ACTION_FINANCE_CREDIT_LAST           =550,
      //--- 
      //--- ACTION_TRADE_CLOSE_POSITIONS
      //--- 
      PARAM_ACTION_TRADE_CLOSE_POSITIONS_TYPES   =551,
      PARAM_ACTION_TRADE_CLOSE_POSITIONS_SYMBOLS =552,
      PARAM_ACTION_TRADE_CLOSE_POSITIONS_PRICE   =553,
      PARAM_ACTION_TRADE_CLOSE_POSITIONS_COMMENT =554,
      PARAM_ACTION_TRADE_CLOSE_POSITIONS_LOGINS  =555,
      PARAM_ACTION_TRADE_CLOSE_POSITIONS_GROUPS  =556,
      PARAM_ACTION_TRADE_CLOSE_POSITIONS_REASON  =557,
      PARAM_ACTION_TRADE_CLOSE_POSITIONS_TRIGGER_LOGIN   =558,
      PARAM_ACTION_TRADE_CLOSE_POSITIONS_TRIGGER_POSITION=559,
      //--- borders
      PARAM_ACTION_TRADE_CLOSE_POSITIONS_FIRST   =551,
      PARAM_ACTION_TRADE_CLOSE_POSITIONS_LAST    =600,
      //--- 
      //--- ACTION_TRADE_CLOSE_ORDERS
      //--- 
      PARAM_ACTION_TRADE_CLOSE_ORDERS_TYPES      =601,
      PARAM_ACTION_TRADE_CLOSE_ORDERS_SYMBOLS    =602,
      PARAM_ACTION_TRADE_CLOSE_ORDERS_COMMENT    =603,
      PARAM_ACTION_TRADE_CLOSE_ORDERS_LOGINS     =604,
      PARAM_ACTION_TRADE_CLOSE_ORDERS_GROUPS     =605,
      PARAM_ACTION_TRADE_CLOSE_ORDERS_TRIGGER_LOGIN=606,
      PARAM_ACTION_TRADE_CLOSE_ORDERS_TRIGGER_ORDER=607,
      //--- borders
      PARAM_ACTION_TRADE_CLOSE_ORDERS_FIRST      =601,
      PARAM_ACTION_TRADE_CLOSE_ORDERS_LAST       =650,
      //--- 
      //--- ACTION_CONFIG_SYMBOL_MOVE
      //--- 
      PARAM_ACTION_CONFIG_SYMBOL_MOVE_PATH       =651,
      //--- borders
      PARAM_ACTION_CONFIG_SYMBOL_MOVE_FIRST      =651,
      PARAM_ACTION_CONFIG_SYMBOL_MOVE_LAST       =700,
      //--- 
      //--- ACTION_PLATFORM_RESTART_SERVER
      //--- 
      PARAM_ACTION_PLATFORM_RESTART_SERVER_LOGIN =701,
      //--- borders
      PARAM_ACTION_PLATFORM_RESTART_SERVER_FIRST =701,
      PARAM_ACTION_PLATFORM_RESTART_SERVER_LAST  =750,
      //--- 
      //--- ACTION_PLATFORM_RESTART_FEED
      //--- 
      PARAM_ACTION_PLATFORM_RESTART_FEED_NAME    =751,
      //--- borders
      PARAM_ACTION_PLATFORM_RESTART_FEED_FIRST   =751,
      PARAM_ACTION_PLATFORM_RESTART_FEED_LAST    =800,
      //--- 
      //--- ACTION_PLATFORM_RESTART_GATEWAY
      //--- 
      PARAM_ACTION_PLATFORM_RESTART_GATEWAY_NAME =801,
      //--- borders
      PARAM_ACTION_PLATFORM_RESTART_GATEWAY_FIRST=801,
      PARAM_ACTION_PLATFORM_RESTART_GATEWAY_LAST =850,
      //--- 
      //--- ACTION_CONFIG_GROUP_UPDATE
      //--- 
      PARAM_ACTION_CONFIG_GROUP_UPDATE_JSON      =851,
      //--- borders
      PARAM_ACTION_CONFIG_GROUP_UPDATE_FIRST     =851,
      PARAM_ACTION_CONFIG_GROUP_UPDATE_LAST      =900,
      //--- 
      //--- ACTION_CONFIG_SYMBOL_UPDATE
      //--- 
      PARAM_ACTION_CONFIG_SYMBOL_UPDATE_JSON     =901,
      //--- borders
      PARAM_ACTION_CONFIG_SYMBOL_UPDATE_FIRST    =901,
      PARAM_ACTION_CONFIG_SYMBOL_UPDATE_LAST     =950,
      //--- 
      //--- ACTION_CONFIG_ROUTING_UPDATE
      //--- 
      PARAM_ACTION_CONFIG_ROUTING_UPDATE_JSON    =951,
      //--- borders
      PARAM_ACTION_CONFIG_ROUTING_UPDATE_FIRST   =951,
      PARAM_ACTION_CONFIG_ROUTING_UPDATE_LAST    =1000,
      //--- 
      //--- ACTION_ACCOUNT_DISABLE
      //--- 
      PARAM_ACTION_ACCOUNT_DISABLE_LOGINS        =1001,
      PARAM_ACTION_ACCOUNT_DISABLE_GROUPS        =1002,
      //--- borders
      PARAM_ACTION_ACCOUNT_DISABLE_FIRST         =1001,
      PARAM_ACTION_ACCOUNT_DISABLE_LAST          =1050,
      //--- 
      //--- ACTION_ACCOUNT_ENABLE
      //--- 
      PARAM_ACTION_ACCOUNT_ENABLE_LOGINS         =1051,
      PARAM_ACTION_ACCOUNT_ENABLE_GROUPS         =1052,
      //--- borders
      PARAM_ACTION_ACCOUNT_ENABLE_FIRST          =1051,
      PARAM_ACTION_ACCOUNT_ENABLE_LAST           =1110,
      //--- 
      //--- ACTION_ACCOUNT_DISABLE_TRADE
      //--- 
      PARAM_ACTION_ACCOUNT_DISABLE_TRADE_LOGINS  =1101,
      PARAM_ACTION_ACCOUNT_DISABLE_TRADE_GROUPS  =1102,
      //--- borders
      PARAM_ACTION_ACCOUNT_DISABLE_TRADE_FIRST   =1101,
      PARAM_ACTION_ACCOUNT_DISABLE_TRADE_LAST    =1150,
      //--- 
      //--- ACTION_ACCOUNT_ENABLE_TRADE
      //--- 
      PARAM_ACTION_ACCOUNT_ENABLE_TRADE_LOGINS   =1151,
      PARAM_ACTION_ACCOUNT_ENABLE_TRADE_GROUPS   =1152,
      //--- borders
      PARAM_ACTION_ACCOUNT_ENABLE_TRADE_FIRST    =1151,
      PARAM_ACTION_ACCOUNT_ENABLE_TRADE_LAST     =1200,
      //--- 
      //--- ACTION_ACCOUNT_DISABLE_EXPERT
      //--- 
      PARAM_ACTION_ACCOUNT_DISABLE_EXPERT_LOGINS =1201,
      PARAM_ACTION_ACCOUNT_DISABLE_EXPERT_GROUPS =1202,
      //--- borders
      PARAM_ACTION_ACCOUNT_DISABLE_EXPERT_FIRST  =1201,
      PARAM_ACTION_ACCOUNT_DISABLE_EXPERT_LAST   =1250,
      //--- 
      //--- ACTION_ACCOUNT_ENABLE_EXPERT
      //--- 
      PARAM_ACTION_ACCOUNT_ENABLE_EXPERT_LOGINS  =1251,
      PARAM_ACTION_ACCOUNT_ENABLE_EXPERT_GROUPS  =1252,
      //--- borders
      PARAM_ACTION_ACCOUNT_ENABLE_EXPERT_FIRST   =1251,
      PARAM_ACTION_ACCOUNT_ENABLE_EXPERT_LAST    =1300,
      //--- 
      //--- ACTION_ACCOUNT_DISABLE_TRAILING
      //--- 
      PARAM_ACTION_ACCOUNT_DISABLE_TRAILING_LOGINS=1301,
      PARAM_ACTION_ACCOUNT_DISABLE_TRAILING_GROUPS=1302,
      //--- borders
      PARAM_ACTION_ACCOUNT_DISABLE_TRAILING_FIRST=1301,
      PARAM_ACTION_ACCOUNT_DISABLE_TRAILING_LAST =1350,
      //--- 
      //--- ACTION_ACCOUNT_ENABLE_TRAILING
      //--- 
      PARAM_ACTION_ACCOUNT_ENABLE_TRAILING_LOGINS=1351,
      PARAM_ACTION_ACCOUNT_ENABLE_TRAILING_GROUPS=1352,
      //--- borders
      PARAM_ACTION_ACCOUNT_ENABLE_TRAILING_FIRST =1351,
      PARAM_ACTION_ACCOUNT_ENABLE_TRAILING_LAST  =1400,
      //--- 
      //--- ACTION_ACCOUNT_FORCE_CHANGE_PASS
      //--- 
      PARAM_ACTION_ACCOUNT_FORCE_CHANGE_PASS_LOGINS=1401,
      PARAM_ACTION_ACCOUNT_FORCE_CHANGE_PASS_GROUPS=1402,
      //--- borders
      PARAM_ACTION_ACCOUNT_FORCE_CHANGE_PASS_FIRST=1401,
      PARAM_ACTION_ACCOUNT_FORCE_CHANGE_PASS_LAST =1450,
      //--- 
      //--- ACTION_ACCOUNT_ARCHIVE
      //--- 
      PARAM_ACTION_ACCOUNT_ARCHIVE_LOGINS         =1451,
      PARAM_ACTION_ACCOUNT_ARCHIVE_GROUPS         =1452,
      //--- borders
      PARAM_ACTION_ACCOUNT_ARCHIVE_FIRST          =1451,
      PARAM_ACTION_ACCOUNT_ARCHIVE_LAST           =1500,
      //--- 
      //--- ACTION_ACCOUNT_LEVERAGE
      //--- 
      PARAM_ACTION_ACCOUNT_LEVERAGE               =1501,
      PARAM_ACTION_ACCOUNT_LEVERAGE_LOGINS        =1502,
      PARAM_ACTION_ACCOUNT_LEVERAGE_GROUPS        =1503,
      //--- borders
      PARAM_ACTION_ACCOUNT_LEVERAGE_FIRST         =1501,
      PARAM_ACTION_ACCOUNT_LEVERAGE_LAST          =1550,
      //--- 
      //--- ACTION_FINANCE_DEPOSIT_PAYOFF
      //--- 
      PARAM_ACTION_FINANCE_DEPOSIT_PAYOFF_LOGINS  =1551,
      PARAM_ACTION_FINANCE_DEPOSIT_PAYOFF_GROUPS  =1552,
      PARAM_ACTION_FINANCE_DEPOSIT_PAYOFF_COMMENT =1553,
      PARAM_ACTION_FINANCE_DEPOSIT_PAYOFF_ACTION  =1554,
      //--- borders
      PARAM_ACTION_FINANCE_DEPOSIT_PAYOFF_FIRST   =1551,
      PARAM_ACTION_FINANCE_DEPOSIT_PAYOFF_LAST    =1600,
      //--- 
      //--- ACTION_FINANCE_CREDIT_PAYOFF
      //--- 
      PARAM_ACTION_FINANCE_CREDIT_PAYOFF_LOGINS   =1601,
      PARAM_ACTION_FINANCE_CREDIT_PAYOFF_GROUPS   =1602,
      PARAM_ACTION_FINANCE_CREDIT_PAYOFF_COMMENT  =1603,
      //--- borders
      PARAM_ACTION_FINANCE_CREDIT_PAYOFF_FIRST    =1601,
      PARAM_ACTION_FINANCE_CREDIT_PAYOFF_LAST     =1650,
      //--- 
      //--- ACTION_EXTERNAL_HTTP_REQUEST
      //--- 
      PARAM_ACTION_EXTERNAL_HTTP_REQUEST_METHOD   =1651,
      PARAM_ACTION_EXTERNAL_HTTP_REQUEST_URL      =1652,
      PARAM_ACTION_EXTERNAL_HTTP_REQUEST_HEADERS  =1653,
      PARAM_ACTION_EXTERNAL_HTTP_REQUEST_DATA     =1654,
      //--- borders
      PARAM_ACTION_EXTERNAL_HTTP_REQUEST_FIRST    =1651,
      PARAM_ACTION_EXTERNAL_HTTP_REQUEST_LAST     =1700,
      //--- 
      //--- ACTION_EXTERNAL_FINTEZA_EVENT
      //--- 
      PARAM_ACTION_EXTERNAL_FINTEZA_EVENT_NAME    =1701,
      PARAM_ACTION_EXTERNAL_FINTEZA_EVENT_VALUE   =1702,
      PARAM_ACTION_EXTERNAL_FINTEZA_EVENT_UNIT    =1703,
      //--- borders
      PARAM_ACTION_EXTERNAL_FINTEZA_EVENT_FIRST   =1701,
      PARAM_ACTION_EXTERNAL_FINTEZA_EVENT_LAST    =1750,
      //--- 
      //--- ACTION_EXTERNAL_APPLICATION
      //--- 
      PARAM_ACTION_EXTERNAL_APPLICATION_PATH      =1751,
      PARAM_ACTION_EXTERNAL_APPLICATION_CMD       =1752,
      //--- borders
      PARAM_ACTION_EXTERNAL_APPLICATION_FIRST     =1751,
      PARAM_ACTION_EXTERNAL_APPLICATION_LAST      =1752,
      //--- borders
      PARAM_FIRST                                 =PARAM_ACTION_MESSAGE_PUSH_TO_LOGINS,
      PARAM_LAST                                  =PARAM_ACTION_EXTERNAL_APPLICATION_LAST
     };
   //--- parameters types
   enum EnParamType
     {
      TYPE_NONE                    =0,
      TYPE_STRING                  =1,
      TYPE_INT                     =2,
      TYPE_UINT                    =3,
      TYPE_FLOAT                   =4,
      TYPE_COLOR                   =5,
      TYPE_MONEY                   =6,
      TYPE_VOLUME                  =7,
      TYPE_DATETIME                =8,
      TYPE_LEVERAGE                =9,
      TYPE_BOOL                    =10,
      TYPE_TIME                    =11,
      TYPE_DATE                    =12,
      TYPE_PERCENT                 =13,
      TYPE_LANGUAGE                =14,
      TYPE_SERVER                  =15,
      TYPE_HTML                    =16,
      //--- borders
      TYPE_FIRST                   =TYPE_NONE,
      TYPE_LAST                    =TYPE_HTML
     };
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTConAutoParam* param)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- param type
   virtual UINT      Param(void) const=0;
   virtual MTAPIRES  Param(const UINT param)=0;
   //--- type
   virtual UINT      ValueType(void) const=0;
   //--- value int
   virtual INT64     ValueInt(void) const=0;
   virtual MTAPIRES  ValueInt(const INT64 value)=0;
   //--- value uint
   virtual UINT64    ValueUInt(void) const=0;
   virtual MTAPIRES  ValueUInt(const UINT64 value)=0;
   //--- value double
   virtual double    ValueDouble(void) const=0;
   virtual MTAPIRES  ValueDouble(const double value)=0;
   //--- value string
   virtual LPCWSTR   ValueString(void) const=0;
   virtual MTAPIRES  ValueString(LPCWSTR value)=0;
   //--- value color
   virtual COLORREF  ValueColor(void) const=0;
   virtual MTAPIRES  ValueColor(const COLORREF value)=0;
   //--- value money
   virtual double    ValueMoney(void) const=0;
   virtual MTAPIRES  ValueMoney(const double value)=0;
   //--- value volume
   virtual UINT64    ValueVolume(void) const=0;
   virtual MTAPIRES  ValueVolume(const UINT64 value)=0;
   //--- value datetime
   virtual INT64     ValueDatetime(void) const=0;
   virtual MTAPIRES  ValueDatetime(const INT64 value)=0;
   //--- value leverage
   virtual INT64     ValueLeverage(void) const=0;
   virtual MTAPIRES  ValueLeverage(const INT64 value)=0;
   //--- value bool
   virtual bool      ValueBool(void) const=0;
   virtual MTAPIRES  ValueBool(const bool value)=0;
   //--- value time
   virtual UINT      ValueTime(void) const=0;
   virtual MTAPIRES  ValueTime(const UINT value)=0;
   //--- value date
   virtual INT64     ValueDate(void) const=0;
   virtual MTAPIRES  ValueDate(const INT64 value)=0;
   //--- value percent
   virtual UINT      ValuePercent(void) const=0;
   virtual MTAPIRES  ValuePercent(const UINT value)=0;
   //--- value language
   virtual UINT      ValueLanguage(void) const=0;
   virtual MTAPIRES  ValueLanguage(const UINT value)=0;
   //--- value server
   virtual UINT64    ValueServer(void) const=0;
   virtual MTAPIRES  ValueServer(const UINT64 value)=0;
   //--- value html
   virtual LPCWSTR   ValueHTML(void) const=0;
   virtual MTAPIRES  ValueHTML(LPCWSTR value)=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTConAutoParam(void) {}
  };

class IMTConAutoAction
  {
public:
   //--- actions
   enum EnActions
     {
      //--- 
      //--- messages
      //--- 
      ACTION_MESSAGE_PUSH             =0,
      ACTION_MESSAGE_INTERNAL         =1,
      ACTION_MESSAGE_SMS              =2,
      ACTION_MESSAGE_EMAIL            =3,
      ACTION_MESSAGE_CHANNEL_TELEGRAM =4,
      //--- messages borders
      ACTION_MESSAGE_FIRST            =0,
      ACTION_MESSAGE_LAST             =1000,
      //--- 
      //--- accounts
      //--- 
      ACTION_ACCOUNT_DISABLE          =1001,
      ACTION_ACCOUNT_ENABLE           =1002,
      ACTION_ACCOUNT_DISABLE_TRADE    =1003,
      ACTION_ACCOUNT_ENABLE_TRADE     =1004,
      ACTION_ACCOUNT_DISABLE_EXPERT   =1005,
      ACTION_ACCOUNT_ENABLE_EXPERT    =1006,
      ACTION_ACCOUNT_DISABLE_TRAILING =1007,
      ACTION_ACCOUNT_ENABLE_TRAILING  =1008,
      ACTION_ACCOUNT_FORCE_CHANGE_PASS=1009,
      ACTION_ACCOUNT_CHANGE_GROUP     =1010,
      ACTION_ACCOUNT_CHANGE_COLOR     =1011,
      ACTION_ACCOUNT_ARCHIVE          =1012,
      ACTION_ACCOUNT_COMMENT          =1013,
      ACTION_ACCOUNT_CLIENT_COMMENT   =1014,
      ACTION_ACCOUNT_LEVERAGE         =1015,
      //--- accounts borders
      ACTION_ACCOUNT_FIRST            =1001,
      ACTION_ACCOUNT_LAST             =2000,
      //--- 
      //--- finances
      //--- 
      ACTION_FINANCE_DEPOSIT          =2001,
      ACTION_FINANCE_BONUS            =2002,
      ACTION_FINANCE_CREDIT           =2003,
      ACTION_FINANCE_DEPOSIT_PAYOFF   =2004,
      ACTION_FINANCE_CREDIT_PAYOFF    =2005,
      ACTION_FINANCE_BONUS_PERCENT    =2006,
      //--- finances borders
      ACTION_FINANCE_FIRST            =2001,
      ACTION_FINANCE_LAST             =3000,
      //--- 
      //--- trade actions
      //--- 
      ACTION_TRADE_CLOSE_POSITIONS    =3001,
      ACTION_TRADE_CLOSE_ORDERS       =4000,
      //--- trade borders
      ACTION_TRADE_FIRST              =3001,
      ACTION_TRADE_LAST               =5000,
      //--- 
      //--- config modifications
      //--- 
      ACTION_CONFIG_GROUP_UPDATE      =5001,
      ACTION_CONFIG_SYMBOL_UPDATE     =5002,
      ACTION_CONFIG_SYMBOL_MOVE       =5003,
      ACTION_CONFIG_ROUTING_UPDATE    =5004,
      ACTION_CONFIG_SERVER_UPDATE     =5005,
      ACTION_CONFIG_GATEWAY_UPDATE    =5006,
      ACTION_CONFIG_DATAFEED_UPDATE   =5007,
      //--- config borders
      ACTION_CONFIG_FIRST             =5001,
      ACTION_CONFIG_LAST              =6000,
      //--- 
      //--- platform actions
      //--- 
      ACTION_PLATFORM_RESTART_SERVER  =6001,
      ACTION_PLATFORM_RESTART_FEED    =6002,
      ACTION_PLATFORM_RESTART_GATEWAY =6003,
      ACTION_PLATFORM_SYNC_HISTORY    =6004,
      ACTION_PLATFORM_LIVE_UPDATE     =6005,
      //--- platform borders
      ACTION_PLATFORM_FIRST           =6001,
      ACTION_PLATFORM_LAST            =7000,
      //---
      //--- external actions
      //--- 
      ACTION_EXTERNAL_APPLICATION     =7001,
      ACTION_EXTERNAL_PLUGIN          =7002,
      ACTION_EXTERNAL_HTTP_REQUEST    =7003,
      ACTION_EXTERNAL_FINTEZA_EVENT   =7004,
      //--- external borders
      ACTION_EXTERNAL_FIRST           =7001,
      ACTION_EXTERNAL_LAST            =8000,
      //--- common borders
      ACTION_FIRST                    =ACTION_MESSAGE_FIRST,
      ACTION_LAST                     =ACTION_EXTERNAL_LAST
     };
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTConAutoAction* action)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- action type
   virtual UINT      Action(void) const=0;
   virtual MTAPIRES  Action(const UINT condition)=0;
   //--- name
   virtual LPCWSTR   Name(void) const=0;
   virtual MTAPIRES  Name(LPCWSTR name)=0;
   //--- accounts
   virtual MTAPIRES  ParamAdd(IMTConAutoParam* param)=0;
   virtual MTAPIRES  ParamUpdate(const UINT pos,const IMTConAutoParam* param)=0;
   virtual MTAPIRES  ParamDelete(const UINT pos)=0;
   virtual MTAPIRES  ParamClear(void)=0;
   virtual MTAPIRES  ParamShift(const UINT pos,const int shift)=0;
   virtual UINT      ParamTotal(void) const=0;
   virtual MTAPIRES  ParamNext(const UINT pos,IMTConAutoParam* param) const=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTConAutoAction(void) {}
  };

class IMTConAutomation
  {
public:
   //--- flags
   enum EnFlags
     {
      FLAG_NONE                       =0x00000000,  // none
      FLAG_ENABLED                    =0x00000001,  // enabled
      FLAG_FOLDER                     =0x00000002,  // folder
      //--- borders
      FLAG_ALL                        =FLAG_ENABLED|FLAG_FOLDER
     };
   //--- automation trigger types
   enum EnTriggers
     {
      //--- 
      //--- time triggers
      //--- 
      TRIGGER_SCHEDULE_SCHEDULE        =0,      // on schedule
      TRIGGER_SCHEDULE_ACCOUNTS        =1,      // by trade accounts base
      TRIGGER_SCHEDULE_POSITIONS       =2,      // by trade positions base
      TRIGGER_SCHEDULE_ORDERS          =3,      // by trade orders base
      //---   SCHEDULE
      TRIGGER_SCHEDULE_FIRST           =0,
      TRIGGER_SCHEDULE_LAST            =1000,
      //--- 
      //--- connection triggers
      //--- 
      TRIGGER_CONNECTIONS_LOGIN           =1001,   // login
      TRIGGER_CONNECTIONS_LOGIN_FIRST     =1002,   // first login
      TRIGGER_CONNECTIONS_LOGOUT          =1003,   // logout
      TRIGGER_CONNECTIONS_AUTHORIZE_FAIL  =1004,   // authorization fail
      //--- connection triggers borders
      TRIGGER_CONNECTIONS_FIRST        =1001,
      TRIGGER_CONNECTIONS_LAST         =2000,
      //---                            
      //--- account triggers           
      //---                            
      TRIGGER_ACCOUNT_CREATE           =2001,   // creation
      //--- account triggers borders   
      TRIGGER_ACCOUNT_FIRST            =2001,
      TRIGGER_ACCOUNT_LAST             =3000,
      //--- 
      //--- finance triggers
      //--- 
      TRIGGER_FINANCE_DEPOSIT_IN       =3001,   // deposit in
      TRIGGER_FINANCE_DEPOSIT_IN_FIRST =3002,   // first deposit in
      TRIGGER_FINANCE_DEPOSIT_OUT      =3003,   // deposit out
      TRIGGER_FINANCE_DEPOSIT_OUT_FIRST=3004,   // first deposit out
      TRIGGER_FINANCE_CREDIT_IN        =3005,   // credit in
      TRIGGER_FINANCE_CREDIT_IN_FIRST  =3006,   // first credit in
      TRIGGER_FINANCE_CREDIT_OUT       =3007,   // credit out
      //--- finance triggers borders
      TRIGGER_FINANCE_FIRST            =3001,
      TRIGGER_FINANCE_LAST             =4000,
      //--- 
      //--- trade triggers
      //--- 
      TRIGGER_TRADE_MARGIN_CALL        =4001,
      TRIGGER_TRADE_STOP_OUT           =4002,
      TRIGGER_TRADE_POSITION_OPEN      =4003,
      TRIGGER_TRADE_POSITION_INCREASE  =4004,
      TRIGGER_TRADE_POSITION_DECREASE  =4005,
      TRIGGER_TRADE_POSITION_CLOSE     =4006,
      TRIGGER_TRADE_POSITION_REVERSE   =4007,
      //--- trade triggers borders     
      TRIGGER_TRADE_FIRST              =4001,
      TRIGGER_TRADE_LAST               =5000,
      //--- 
      //--- price triggers
      //--- 
      TRIGGER_PRICES_GAP_STARTED       =5001,
      TRIGGER_PRICES_GAP_FINISHED      =5002,
      TRIGGER_PRICES_DELAYED           =5003,
      TRIGGER_PRICES_RESUMED           =5004,
      //--- price triggers borders     
      TRIGGER_PRICES_FIRST             =5001,
      TRIGGER_PRICES_LAST              =6000,
      //--- 
      //--- platform triggers
      //--- 
      TRIGGER_PLATFORM_MONITOR         =6001, // server monitoring
      TRIGGER_PLATFORM_CONNECT         =6002, // server connect
      TRIGGER_PLATFORM_DISCONNECT      =6003, // server disconnect
      TRIGGER_PLATFORM_FAILOVER        =6004, // server failover
      //--- platform triggers borders  
      TRIGGER_PLATFORM_FIRST           =6001,
      TRIGGER_PLATFORM_LAST            =7000,
      //--- 
      //--- external triggers
      //--- 
      TRIGGER_EXTERNAL_API             =7001,
      TRIGGER_EXTERNAL_FIRST           =7001,
      TRIGGER_EXTERNAL_LAST            =8000,
      //---                            
      //--- internal mail triggers      
      //---                            
      TRIGGER_MESSAGE_CLIENT_SEND      =8001,
      TRIGGER_MESSAGE_CLIENT_READ      =8002,
      //---                            
      TRIGGER_MESSAGE_FIRST            =8001,
      TRIGGER_MESSAGE_LAST             =9000,
      //--- 
      //--- gateway events
      //--- 
      TRIGGER_GATEWAY_CONNECT         =9001,
      TRIGGER_GATEWAY_DISCONNECT      =9002,
      //---  
      TRIGGER_GATEWAY_FIRST           =9001,
      TRIGGER_GATEWAY_LAST            =10000,
      //--- 
      //--- datafeed events
      //--- 
      TRIGGER_DATAFEED_CONNECT        =10001,
      TRIGGER_DATAFEED_DISCONNECT     =10002,
      //---  
      TRIGGER_DATAFEED_FIRST          =10001,
      TRIGGER_DATAFEED_LAST           =11000,
      //--- 
      //--- KYC
      //--- 
      TRIGGER_KYC_START               =11001,
      TRIGGER_KYC_CONFIRM             =11002,
      TRIGGER_KYC_REJECT              =11003,
      //---  
      TRIGGER_KYC_FIRST               =11001,
      TRIGGER_KYC_LAST                =12000,
      //--- external triggers borders  
      TRIGGER_FIRST                   =TRIGGER_SCHEDULE_FIRST,
      TRIGGER_LAST                    =TRIGGER_KYC_LAST
     };
   //--- weekdays flags
   enum EnTriggerWeekdays
     {
      TRIGGER_WEEKDAY_SUN        =0x00000001,
      TRIGGER_WEEKDAY_MON        =0x00000002,
      TRIGGER_WEEKDAY_TUE        =0x00000004,
      TRIGGER_WEEKDAY_WED        =0x00000008,
      TRIGGER_WEEKDAY_THU        =0x00000010,
      TRIGGER_WEEKDAY_FRI        =0x00000020,
      TRIGGER_WEEKDAY_SAT        =0x00000040,
      //--- flag borders
      TRIGGER_WEEKDAY_NONE       =0,
      TRIGGER_WEEKDAY_ALL        =TRIGGER_WEEKDAY_SUN|TRIGGER_WEEKDAY_MON|TRIGGER_WEEKDAY_TUE|TRIGGER_WEEKDAY_WED|TRIGGER_WEEKDAY_THU|TRIGGER_WEEKDAY_FRI|TRIGGER_WEEKDAY_SAT,
      TRIGGER_WEEKDAY_FIRST      =TRIGGER_WEEKDAY_SUN,
      TRIGGER_WEEKDAY_LAST       =TRIGGER_WEEKDAY_SAT
     };
   //--- months flags
   enum EnTriggerMonths
     {
      TRIGGER_MONTHS_JAN      =0x00000001,
      TRIGGER_MONTHS_FEB      =0x00000002,
      TRIGGER_MONTHS_MAR      =0x00000004,
      TRIGGER_MONTHS_APR      =0x00000008,
      TRIGGER_MONTHS_MAY      =0x00000010,
      TRIGGER_MONTHS_JUN      =0x00000020,
      TRIGGER_MONTHS_JUL      =0x00000040,
      TRIGGER_MONTHS_AUG      =0x00000080,
      TRIGGER_MONTHS_SEP      =0x00000100,
      TRIGGER_MONTHS_OCT      =0x00000200,
      TRIGGER_MONTHS_NOV      =0x00000400,
      TRIGGER_MONTHS_DEC      =0x00000800,
      //--- flag borders
      TRIGGER_MONTHS_NONE     =0,
      TRIGGER_MONTHS_ALL      =TRIGGER_MONTHS_JAN |TRIGGER_MONTHS_FEB|TRIGGER_MONTHS_MAR|TRIGGER_MONTHS_APR|TRIGGER_MONTHS_MAY|TRIGGER_MONTHS_JUN|TRIGGER_MONTHS_JUL|
      TRIGGER_MONTHS_AUG |TRIGGER_MONTHS_SEP|TRIGGER_MONTHS_OCT|TRIGGER_MONTHS_NOV|TRIGGER_MONTHS_DEC,
      TRIGGER_MONTHS_FIRST    =TRIGGER_MONTHS_JAN,
      TRIGGER_MONTHS_LAST     =TRIGGER_MONTHS_DEC
     };
   //--- month days flags
   enum EnTriggerMonthDays : UINT
     {
      TRIGGER_MONTHDAY_1      =0x00000001,
      TRIGGER_MONTHDAY_2      =0x00000002,
      TRIGGER_MONTHDAY_3      =0x00000004,
      TRIGGER_MONTHDAY_4      =0x00000008,
      TRIGGER_MONTHDAY_5      =0x00000010,
      TRIGGER_MONTHDAY_6      =0x00000020,
      TRIGGER_MONTHDAY_7      =0x00000040,
      TRIGGER_MONTHDAY_8      =0x00000080,
      TRIGGER_MONTHDAY_9      =0x00000100,
      TRIGGER_MONTHDAY_10     =0x00000200,
      TRIGGER_MONTHDAY_11     =0x00000400,
      TRIGGER_MONTHDAY_12     =0x00000800,
      TRIGGER_MONTHDAY_13     =0x00001000,
      TRIGGER_MONTHDAY_14     =0x00002000,
      TRIGGER_MONTHDAY_15     =0x00004000,
      TRIGGER_MONTHDAY_16     =0x00008000,
      TRIGGER_MONTHDAY_17     =0x00010000,
      TRIGGER_MONTHDAY_18     =0x00020000,
      TRIGGER_MONTHDAY_19     =0x00040000,
      TRIGGER_MONTHDAY_20     =0x00080000,
      TRIGGER_MONTHDAY_21     =0x00100000,
      TRIGGER_MONTHDAY_22     =0x00200000,
      TRIGGER_MONTHDAY_23     =0x00400000,
      TRIGGER_MONTHDAY_24     =0x00800000,
      TRIGGER_MONTHDAY_25     =0x01000000,
      TRIGGER_MONTHDAY_26     =0x02000000,
      TRIGGER_MONTHDAY_27     =0x04000000,
      TRIGGER_MONTHDAY_28     =0x08000000,
      TRIGGER_MONTHDAY_29     =0x10000000,
      TRIGGER_MONTHDAY_30     =0x20000000,
      TRIGGER_MONTHDAY_31     =0x40000000,
      //--- flag borders
      TRIGGER_MONTHDAY_NONE   =0,
      TRIGGER_MONTHDAY_ALL    =0x7FFFFFFF,
      TRIGGER_MONTHDAY_FIRST  =TRIGGER_MONTHDAY_1,
      TRIGGER_MONTHDAY_LAST   =TRIGGER_MONTHDAY_31
     };
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTConAutomation* automation)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- uniq ids
   virtual UINT64    ID(void) const=0;
   virtual UINT64    ParentID(void) const=0;
   //--- name
   virtual LPCWSTR   Name(void) const=0;
   virtual MTAPIRES  Name(LPCWSTR name)=0;
   //--- trigger type
   virtual UINT      Trigger(void) const=0;
   virtual MTAPIRES  Trigger(const UINT trigger)=0;
   //--- trigger flags
   virtual UINT64    Flags(void) const=0;
   virtual MTAPIRES  Flags(const UINT64 flags)=0;
   //--- time start
   virtual INT64     TimeStart(void) const=0;
   virtual MTAPIRES  TimeStart(const INT64 start)=0;
   //--- time expire
   virtual INT64     TimeExpire(void) const=0;
   virtual MTAPIRES  TimeExpire(const INT64 expire)=0;
   //--- time weekdays flags
   virtual UINT      TimeWeekdays(void) const=0;
   virtual MTAPIRES  TimeWeekdays(const UINT weekdays)=0;
   //--- time months flags
   virtual UINT      TimeMonths(void) const=0;
   virtual MTAPIRES  TimeMonths(const UINT months)=0;
   //--- time monthdays flags
   virtual UINT      TimeMonthdays(void) const=0;
   virtual MTAPIRES  TimeMonthdays(const UINT monthdays)=0;
   //--- event pause - minutes part (0-59)
   virtual UINT      EventPauseMinutes(void) const=0;
   virtual MTAPIRES  EventPauseMinutes(const UINT minutes)=0;
   //--- event pause - hours part (0-23)
   virtual UINT      EventPauseHours(void) const=0;
   virtual MTAPIRES  EventPauseHours(const UINT hours)=0;
   //--- event pause - hours part (0-30)
   virtual UINT      EventPauseDays(void) const=0;
   virtual MTAPIRES  EventPauseDays(const UINT days)=0;
   //--- event repeats
   virtual UINT      EventRepeats(void) const=0;
   virtual MTAPIRES  EventRepeats(const UINT repeats)=0;
   //--- conditions
   virtual MTAPIRES  ConditionAdd(IMTConAutoCondition* condition)=0;
   virtual MTAPIRES  ConditionUpdate(const UINT pos,const IMTConAutoCondition* condition)=0;
   virtual MTAPIRES  ConditionDelete(const UINT pos)=0;
   virtual MTAPIRES  ConditionClear(void)=0;
   virtual MTAPIRES  ConditionShift(const UINT pos,const int shift)=0;
   virtual UINT      ConditionTotal(void) const=0;
   virtual MTAPIRES  ConditionNext(const UINT pos,IMTConAutoCondition* condition) const=0;
   //--- actions
   virtual MTAPIRES  ActionAdd(IMTConAutoAction* action)=0;
   virtual MTAPIRES  ActionUpdate(const UINT pos,const IMTConAutoAction* action)=0;
   virtual MTAPIRES  ActionDelete(const UINT pos)=0;
   virtual MTAPIRES  ActionClear(void)=0;
   virtual MTAPIRES  ActionShift(const UINT pos,const int shift)=0;
   virtual UINT      ActionTotal(void) const=0;
   virtual MTAPIRES  ActionNext(const UINT pos,IMTConAutoAction* action) const=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTConAutomation(void) {}
  };

class IMTConAutomationSink
  {
public:
   virtual void      OnAutomationAdd(const IMTConAutomation*    /*config*/) {  }
   virtual void      OnAutomationUpdate(const IMTConAutomation* /*config*/) {  }
   virtual void      OnAutomationDelete(const IMTConAutomation* /*config*/) {  }
   virtual void      OnAutomationSync(void)                                 {  }
  };

class IMTConSubscriptionSymbol
  {
public:
   //--- subscription level
   enum EnLevel
     {
      LEVEL_DELAYED            =0,      // delayed
      LEVEL_REALTIME_LEVEL_1   =1,      // Level 1 realtime, best Bid/Ask
      LEVEL_REALTIME_LEVEL_2   =2,      // Level 2 realtime
      //--- enumeration borders
      LEVEL_FIRST              =LEVEL_DELAYED,
      LEVEL_LAST               =LEVEL_REALTIME_LEVEL_2,
     };
   //--- available tick history
   enum EnTickHistory
     {
      TICK_HISTORY_NONE        =0,
      TICK_HISTORY_LAST_DAY    =1,
      TICK_HISTORY_LAST_WEEK   =2,
      TICK_HISTORY_LAST_MONTH  =3,
      TICK_HISTORY_LAST_3MONTHS=4,
      TICK_HISTORY_LAST_6MONTHS=5,
      TICK_HISTORY_LAST_YEAR   =6,
      TICK_HISTORY_LAST_2YEARS =7,
      TICK_HISTORY_LAST_3YEARS =8,
      TICK_HISTORY_LAST_5YEARS =9,
      TICK_HISTORY_ALL         =10,
      //--- enumeration borders
      TICK_HISTORY_FIRST       =TICK_HISTORY_NONE,
      TICK_HISTORY_LAST        =TICK_HISTORY_ALL,
     };
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTConSubscriptionSymbol* symbol)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- symbols mask
   virtual LPCWSTR   Symbol(void) const=0;
   virtual MTAPIRES  Symbol(LPCWSTR symbols)=0;
   //--- ticks level
   virtual UINT      Level(void) const=0;
   virtual MTAPIRES  Level(const UINT level)=0;
   //--- ticks history mode
   virtual UINT      TickHistory(void) const=0;
   virtual MTAPIRES  TickHistory(const UINT mode)=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTConSubscriptionSymbol(void) {}
  };

class IMTConSubscriptionNews
  {
public:
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTConSubscriptionNews* news)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- symbols mask
   virtual LPCWSTR   Category(void) const=0;
   virtual MTAPIRES  Category(LPCWSTR category)=0;
   //--- language
   virtual UINT      Language(void) const=0;
   virtual MTAPIRES  Language(const UINT language)=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTConSubscriptionNews(void) {}
  };

class IMTConSubscription
  {
public:
   //--- type
   enum EnType
     {
      TYPE_SUBSCRIPTION          =0,
      TYPE_FOLDER                =1,
      //--- enumeration borders
      TYPE_FIRST                 =TYPE_SUBSCRIPTION,
      TYPE_LAST                  =TYPE_FOLDER
     };
   //--- period
   enum EnPeriod
     {
      PERIOD_ONCE                =0,
      PERIOD_DAILY               =1,
      PERIOD_WEEKLY              =2,
      PERIOD_MONTHLY             =3,
      PERIOD_QUARTERLY           =4,
      PERIOD_ANNUAL              =5,
      PERIOD_CUSTOM              =100,
      //--- enumeration borders
      PERIOD_FIRST               =PERIOD_ONCE,
      PERIOD_LAST                =PERIOD_CUSTOM
     };
   //--- free subscription period
   enum EnFreePeriod
     {
      FREE_PERIOD_NONE           =0,
      FREE_PERIOD_DAY            =1,
      FREE_PERIOD_WEEK           =2,
      FREE_PERIOD_MONTH          =3,
      FREE_PERIOD_QUARTER        =4,
      FREE_PERIOD_YEAR           =5,
      FREE_PERIOD_CUSTOM         =100,
      //--- enumeration borders
      FREE_PERIOD_FIRST          =FREE_PERIOD_NONE,
      FREE_PERIOD_LAST           =FREE_PERIOD_CUSTOM
     };
   //--- flags
   enum EnFlags
     {
      FLAG_NONE                  =0x00000000,  // none
      FLAG_ENABLE                =0x00000001,  // enabled
      FLAG_AUTO_PROLONG          =0x00000002,  // auto prolong enabled
      //--- enumeration borders
      FLAG_ALL                   =FLAG_ENABLE|FLAG_AUTO_PROLONG
     };
   //--- client control modes
   enum EnControlMode
     {
      CONTROL_FULL               =0,            // full
      CONTROL_UNSUBCRIBE         =1,            // only unsubscribe
      CONTROL_VIEW               =2,            // only view
      CONTROL_HIDDEN             =3,            // hidden
      //--- enumeration borders
      CONTROL_FIRST              =CONTROL_FULL,
      CONTROL_LAST               =CONTROL_HIDDEN,
     };
   //--- available pictures enumeration
   enum EnImageType
     {
      IMAGE_CUSTOM                      =0,
      IMAGE_DEFAULT                     =1,
      IMAGE_STOCK_EXCHANGE              =2,
      IMAGE_AUDIT                       =3,
      IMAGE_REPORT                      =4,
      IMAGE_PERSONAL_MANAGER            =5,
      IMAGE_DOCUMENTS_DELIVERY          =6,
      IMAGE_DOCUMENTS_STORING           =7,
      IMAGE_TRANSFER                    =8,
      IMAGE_CONVERSION                  =9,
      //---
      IMAGE_NASDAQ                      =1000,
      IMAGE_TMX_GROUP                   =1001,
      IMAGE_CANADIAN_SECURITIES_EXCHANGE=1002,
      IMAGE_CBOE                        =1003,
      IMAGE_CBOE_FUTURES_EXCHANGE       =1004,
      IMAGE_MEXICAN_DERIVATIVES         =1005,
      IMAGE_FUND_SERV                   =1006,
      IMAGE_CME_CBOT                    =1007,
      IMAGE_CME                         =1008,
      IMAGE_GLOBAL_OTC                  =1009,
      IMAGE_ICE_FUTURES                 =1010,
      IMAGE_IEX_GROUP                   =1011,
      IMAGE_ISE_OPTIONS                 =1012,
      IMAGE_NYSE                        =1013,
      IMAGE_ONE_CHICAGO                 =1014,
      IMAGE_OTC_MARKETS                 =1015,
      IMAGE_BOND_RATINGS                =1016,
      IMAGE_US_REG_NMS                  =1017,
      IMAGE_BOVESPA                     =1018,
      IMAGE_VIENNA_STOCK_EXCHANGE       =1019,
      IMAGE_EURONEXT                    =1020,
      IMAGE_GERMAN_ETFS                 =1021,
      IMAGE_BOLSA_DE_MADRID             =1022,
      IMAGE_STOXX                       =1023,
      IMAGE_MEFF                        =1024,
      IMAGE_BORSA_ITALIANA              =1025,
      IMAGE_EUREX                       =1026,
      IMAGE_MOSCOW_EXCHANGE             =1027,
      IMAGE_NORDIC_DERIVATIVES_EXCHANGE =1028,
      IMAGE_OSLO_STOCK_EXCHANGE         =1029,
      IMAGE_PRAGUE_STOCK_EXCHANGE       =1030,
      IMAGE_SIX_GROUP                   =1031,
      IMAGE_XETRA                       =1032,
      IMAGE_BOERSE_STUTTGART            =1033,
      IMAGE_TURQUOISE                   =1034,
      IMAGE_LONDON_STOCK_EXCHANGE       =1035,
      IMAGE_WARSAW_STOCK_EXCHANGE       =1036,
      IMAGE_BUDAPEST_STOCK_EXCHANGE     =1037,
      IMAGE_BATS_CHI_X_EUROPE           =1038,
      IMAGE_LONDON_METAL_EXCHANGE       =1039,
      IMAGE_EUROPEAN_MUTUAL_FUNDS       =1040,
      IMAGE_TEL_AVIV_STOCK_EXCHANGE     =1041,
      IMAGE_JOHANNESBURG_STOCK_EXCHANGE =1042,
      IMAGE_HANG_SENG_BANK              =1043,
      IMAGE_CHI_X_AUSTRALIA             =1044,
      IMAGE_HKEX                        =1045,
      IMAGE_JPX                         =1046,
      IMAGE_SHANGHAI_STOCK_EXCHANGE     =1047,
      IMAGE_SHENZHEN_STOCK_EXCHANGE     =1048,
      IMAGE_SINGAPORE_EXCHANGE          =1049,
      IMAGE_AUSTRALIAN_STOCK_EXCHANGE   =1050,
      IMAGE_KOREA_STOCK_EXCHANGE        =1051,
      IMAGE_A2X_MARKETS                 =1052,
      IMAGE_ASCE                        =1053,  // Abuja Securities and Commodities Exchange
      IMAGE_ALTX_EAST_AFRICA_EXCHANGE   =1054,
      IMAGE_AMMAN_STOCK_EXCHANGE        =1055,
      IMAGE_ARMENIA_SECURITIES_EXCHANGE =1056,
      IMAGE_ATHENS_STOCK_EXCHANGE       =1057,
      IMAGE_BAKU_STOCK_EXCHANGE         =1058,
      IMAGE_BANJA_LUKA_STOCK_EXCHANGE   =1059,
      IMAGE_BERMUDA_STOCK_EXCHANGE      =1060,
      IMAGE_BOLIVIA_STOCK_EXCHANGE      =1061,
      IMAGE_BVRD                        =1062, // Bolsa de Valores Republica Dominicana
      IMAGE_BOLSAS_Y_MERCADOS_ESPANOLES =1063,
      IMAGE_BOMBAY_STOCK_EXCHANGE       =1064,
      IMAGE_BORSA_ISTANBUL              =1065,
      IMAGE_BOTSWANA_STOCK_EXCHANGE     =1067,
      IMAGE_BOURSA_KUWAIT               =1068,
      IMAGE_BVMAC                       =1069, // Bourse des Valeurs Mobilieres de l Afrique Centrale
      IMAGE_BOURSE_DE_TUNIS             =1070,
      IMAGE_BRVM                        =1071, // Bourse Regionale des Valeurs Mobilieres SA
      IMAGE_BRAZIL_STOCK_EXCHANGE       =1072,
      IMAGE_BUCHAREST_STOCK_EXCHANGE    =1073,
      IMAGE_BUENOS_AIRES_STOCK_EXCHANGE =1074,
      IMAGE_BULGARIAN_STOCK_EXCHANGE    =1075,
      IMAGE_BURSA_MALAYSIA              =1076,
      IMAGE_CALCUTTA_STOCK_EXCHANGE     =1077,
      IMAGE_CARACAS_STOCK_EXCHANGE      =1078,
      IMAGE_CASABLANCA_STOCK_EXCHANGE   =1079,
      IMAGE_COLOMBIA_STOCK_EXCHANGE     =1080,
      IMAGE_COLOMBO_STOCK_EXCHANGE      =1081,
      IMAGE_CYPRUS_STOCK_EXCHANGE       =1082,
      IMAGE_DALIAN_COMMODITY_EXCHANGE   =1083,
      IMAGE_DAMASCUS_SECURITIES_EXCHANGE=1084,
      IMAGE_DAR_ES_SALAAM_STOCK_EXCHANGE=1085,
      IMAGE_DEUTSCHE_BOERSE             =1086,
      IMAGE_DHAKA_STOCK_EXCHANGE        =1087,
      IMAGE_DOHA_SECURITIES_MARKET      =1088,
      IMAGE_DOUALA_STOCK_EXCHANGE       =1089,
      IMAGE_DUBAI_FINANCIAL_MARKET      =1090,
      IMAGE_DUTCH_CARIBBEAN_SECURITIES_EXCHANGE=1091,
      IMAGE_EGYPTIAN_EXCHANGE           =1092,
      IMAGE_ESWATINI_STOCK_MARKET       =1093,
      IMAGE_FRANKFURT_STOCK_EXCHANGE    =1094,
      IMAGE_GEORGIAN_STOCK_EXCHANGE     =1095,
      IMAGE_GHANA_STOCK_EXCHANGE        =1096,
      IMAGE_INDONESIA_STOCK_EXCHANGE    =1097,
      IMAGE_IRAN_FARA_BOURSE            =1098,
      IMAGE_IRAN_MERCANTILE_EXCHANGE    =1099,
      IMAGE_ISLAMABAD_STOCK_EXCHANGE    =1100,
      IMAGE_JAMAICA_STOCK_EXCHANGE      =1101,
      IMAGE_KAZAKHSTAN_STOCK_EXCHANGE   =1102,
      IMAGE_KHARTOUM_STOCK_EXCHANGE     =1103,
      IMAGE_LAHORE_STOCK_EXCHANGE       =1104,
      IMAGE_LIBYAN_STOCK_MARKET         =1105,
      IMAGE_LJUBLJANA_STOCK_EXCHANGE    =1106,
      IMAGE_LUSAKA_STOCK_EXCHANGE       =1107,
      IMAGE_LUXEMBOURG_STOCK_EXCHANGE   =1108,
      IMAGE_MALAWI_STOCK_EXCHANGE       =1109,
      IMAGE_MIAMI_INTERNATIONAL_SECURITIES_EXCHANGE=1110,
      IMAGE_MONGOLIAN_STOCK_EXCHANGE    =1111,
      IMAGE_MULTI_COMMODITY_EXCHANGE    =1112,
      IMAGE_MUSCAT_SECURITIES_MARKET    =1113,
      IMAGE_NAIROBI_SECURITIES_EXCHANGE =1114,
      IMAGE_NCDEX                       =1115, // National Commodity and Derivatives Exchange
      IMAGE_NATIONAL_STOCK_EXCHANGE     =1116,
      IMAGE_NEO_EXCHANGE                =1117,
      IMAGE_NEPAL_STOCK_EXCHANGE        =1118,
      IMAGE_NEW_ZEALAND_EXCHANGE        =1119,
      IMAGE_NIGERIAN_STOCK_EXCHANGE     =1120,
      IMAGE_NXCHANGE                    =1121,
      IMAGE_PAKISTAN_STOCK_EXCHANGE     =1122,
      IMAGE_PALESTINE_SECURITIES_EXCHANGE=1123,
      IMAGE_PFTS_UKRAINE_STOCK_EXCHANGE =1124,
      IMAGE_PHILIPPINE_DEALING_EXCHANGE =1125,
      IMAGE_PHILIPPINE_STOCK_EXCHANGE   =1126,
      IMAGE_SAINT_PETERSBURG_STOCK_EXCHANGE=1127,
      IMAGE_SVGEX                       =1128, // Saint Vincent and the Grenadines Securities Exchange
      IMAGE_SANTIAGO_STOCK_EXCHANGE_MILA=1129,
      IMAGE_STOCK_EXCHANGE_OF_MAURITIUS =1130,
      IMAGE_STOCK_EXCHANGE_OF_THAILAND  =1131,
      IMAGE_TADAWUL                     =1132,
      IMAGE_TAIWAN_STOCK_EXCHANGE       =1133,
      IMAGE_TEHRAN_STOCK_EXCHANGE       =1134,
      IMAGE_TIRANA_STOCK_EXCHANGE       =1135,
      IMAGE_TOKYO_STOCK_EXCHANGE        =1136,
      IMAGE_UGANDA_SECURITIES_EXCHANGE  =1137,
      IMAGE_UKRAINIAN_EXCHANGE          =1138,
      IMAGE_ZAGREB_STOCK_EXCHANGE       =1139,
      IMAGE_ZIMBABWE_STOCK_EXCHANGE     =1140,
      IMAGE_BALTIC_EXCHANGE             =1141,
      //--- enumeration borders
      IMAGE_FIRST                       =IMAGE_CUSTOM,
      IMAGE_LAST                        =IMAGE_BALTIC_EXCHANGE,
     };
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTConSubscription* iface)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- uniq ids
   virtual UINT64    ID(void) const=0;
   virtual UINT64    ParentID(void) const=0;
   //--- id of the subscription that the current one depends on
   virtual UINT64    DependsID(void) const=0;
   virtual MTAPIRES  DependsID(const UINT64 depends_id)=0;
   //--- name
   virtual LPCWSTR   Name(void) const=0;
   virtual MTAPIRES  Name(LPCWSTR name)=0;
   //--- type
   virtual UINT      Type(void) const=0;
   virtual MTAPIRES  Type(const UINT type)=0;
   //--- image
   virtual UINT      Image(void) const=0;
   virtual MTAPIRES  Image(const UINT image)=0;
   //--- description
   virtual LPCWSTR   Description(void) const=0;
   virtual MTAPIRES  Description(LPCWSTR description)=0;
   //--- description URL
   virtual LPCWSTR   URLDescription(void) const=0;
   virtual MTAPIRES  URLDescription(const LPCWSTR url)=0;
   //--- agreement URL
   virtual LPCWSTR   URLAgreement(void) const=0;
   virtual MTAPIRES  URLAgreement(const LPCWSTR url)=0;
   //--- control mode
   virtual UINT      ControlMode(void) const=0;
   virtual MTAPIRES  ControlMode(const UINT mode)=0;
   //--- period mode
   virtual UINT      PeriodMode(void) const=0;
   virtual MTAPIRES  PeriodMode(const UINT mode)=0;
   //--- custom period in days
   virtual UINT      PeriodCustom(void) const=0;
   virtual MTAPIRES  PeriodCustom(const UINT days)=0;
   //--- free period mode
   virtual UINT      PeriodFreeMode(void) const=0;
   virtual MTAPIRES  PeriodFreeMode(const UINT mode)=0;
   //--- free custom period in days
   virtual UINT      PeriodFreeCustom(void) const=0;
   virtual MTAPIRES  PeriodFreeCustom(const UINT days)=0;
   //--- flags
   virtual UINT      Flags(void) const=0;
   virtual MTAPIRES  Flags(const UINT flags)=0;
   //--- price
   virtual double    Price(void) const=0;
   virtual MTAPIRES  Price(const double price)=0;
   //--- price professional
   virtual double    PricePro(void) const=0;
   virtual MTAPIRES  PricePro(const double price)=0;
   //--- real cost
   virtual double    PriceCost(void) const=0;
   virtual MTAPIRES  PriceCost(const double price)=0;
   //--- currency
   virtual LPCWSTR   PriceCurrency(void) const=0;
   virtual MTAPIRES  PriceCurrency(LPCWSTR currency)=0;
   //--- allowed country list
   virtual MTAPIRES  CountryAdd(LPCWSTR path)=0;
   virtual MTAPIRES  CountryUpdate(const UINT pos,LPCWSTR path)=0;
   virtual MTAPIRES  CountryShift(const UINT pos,const int shift)=0;
   virtual MTAPIRES  CountryDelete(const UINT pos)=0;
   virtual MTAPIRES  CountryClear(void)=0;
   virtual UINT      CountryTotal(void) const=0;
   virtual LPCWSTR   CountryNext(const UINT pos) const=0;
   //--- allowed groups list
   virtual MTAPIRES  GroupAdd(LPCWSTR path)=0;
   virtual MTAPIRES  GroupUpdate(const UINT pos,LPCWSTR path)=0;
   virtual MTAPIRES  GroupDelete(const UINT pos)=0;
   virtual MTAPIRES  GroupClear(void)=0;
   virtual MTAPIRES  GroupShift(const UINT pos,const int shift)=0;
   virtual UINT      GroupTotal(void) const=0;
   virtual LPCWSTR   GroupNext(const UINT pos) const=0;
   //--- symbols settings
   virtual MTAPIRES  SymbolAdd(IMTConSubscriptionSymbol* symbol)=0;
   virtual MTAPIRES  SymbolUpdate(const UINT pos,const IMTConSubscriptionSymbol* symbol)=0;
   virtual MTAPIRES  SymbolDelete(const UINT pos)=0;
   virtual MTAPIRES  SymbolClear(void)=0;
   virtual MTAPIRES  SymbolShift(const UINT pos,const int shift)=0;
   virtual UINT      SymbolTotal(void) const=0;
   virtual MTAPIRES  SymbolNext(const UINT pos,IMTConSubscriptionSymbol* symbol) const=0;
   //--- symbols settings
   virtual MTAPIRES  NewsAdd(IMTConSubscriptionNews* news)=0;
   virtual MTAPIRES  NewsUpdate(const UINT pos,const IMTConSubscriptionNews* news)=0;
   virtual MTAPIRES  NewsDelete(const UINT pos)=0;
   virtual MTAPIRES  NewsClear(void)=0;
   virtual MTAPIRES  NewsShift(const UINT pos,const int shift)=0;
   virtual UINT      NewsTotal(void) const=0;
   virtual MTAPIRES  NewsNext(const UINT pos,IMTConSubscriptionNews* news) const=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTConSubscription(void) {}
  };

class IMTConSubscriptionSink
  {
public:
   virtual void      OnSubscriptionCfgAdd(const IMTConSubscription*    /*config*/) {  }
   virtual void      OnSubscriptionCfgUpdate(const IMTConSubscription* /*config*/) {  }
   virtual void      OnSubscriptionCfgDelete(const IMTConSubscription* /*config*/) {  }
   virtual void      OnSubscriptionCfgSync(void)                                   {  }
  };

class IMTSubscription
  {
public:
   //---- subscription status
   enum EnStatus
     {
      STATUS_ACTIVE            =0,          // subscription is active
      STATUS_SUSPENDED         =1,          // suspended due "no money"
      STATUS_CANCELED          =2,          // canceled by client
      //---
      STATUS_FIRST             =STATUS_ACTIVE,
      STATUS_LAST              =STATUS_CANCELED
     };
   //---- subscription flags
   enum EnFlags
     {
      FLAG_FREE_PERIOD         =0x01,        // subscription is activated with free period
      //---
      FLAG_NONE                =0x00,
      FLAG_ALL                 =FLAG_FREE_PERIOD
     };

   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTSubscription* obj)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- id
   virtual UINT64    ID(void) const=0;
   //--- client login
   virtual UINT64    Login(void) const=0;
   virtual MTAPIRES  Login(const UINT64 login)=0;
   //--- subscription id
   virtual UINT64    Subscription(void) const=0;
   virtual MTAPIRES  Subscription(const UINT64 subscription_id)=0;
   //--- EnStatus
   virtual UINT      Status(void) const=0;
   virtual MTAPIRES  Status(const UINT status)=0;
   //--- subscribe time
   virtual INT64     TimeSubscribe(void) const=0;
   virtual MTAPIRES  TimeSubscribe(const INT64 time)=0;
   //--- last renewal time
   virtual INT64     TimeRenewal(void) const=0;
   virtual MTAPIRES  TimeRenewal(const INT64 time)=0;
   //--- expire time
   virtual INT64     TimeExpire(void) const=0;
   virtual MTAPIRES  TimeExpire(const INT64 time)=0;
   //--- flags
   virtual UINT      Flags(void) const=0;
   virtual MTAPIRES  Flags(const UINT flags)=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTSubscription(void) {}
  };

class IMTSubscriptionArray
  {
public:
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTSubscriptionArray* array)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- add
   virtual MTAPIRES  Add(IMTSubscription* record)=0;
   virtual MTAPIRES  AddCopy(const IMTSubscription* record)=0;
   virtual MTAPIRES  Add(IMTSubscriptionArray* array)=0;
   virtual MTAPIRES  AddCopy(const IMTSubscriptionArray* array)=0;
   //--- delete & detach
   virtual MTAPIRES  Delete(const UINT pos)=0;
   virtual IMTSubscription* Detach(const UINT pos)=0;
   //--- update
   virtual MTAPIRES  Update(const UINT pos,IMTSubscription* record)=0;
   virtual MTAPIRES  UpdateCopy(const UINT pos,const IMTSubscription* record)=0;
   virtual MTAPIRES  Shift(const UINT pos,const int shift)=0;
   //--- data access
   virtual UINT      Total(void) const=0;
   virtual IMTSubscription*  Next(const UINT index) const=0;
   //--- sorting and search
   virtual MTAPIRES  Sort(MTSortFunctionPtr sort_function)=0;
   virtual int       Search(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchGreatOrEq(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchGreater(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchLessOrEq(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchLess(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchLeft(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchRight(const void *key,MTSortFunctionPtr sort_function) const=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTSubscriptionArray(void) {}
  };

class IMTSubscriptionHistory
  {
public:
   //---- actions
   enum EnAction
     {
      ACTION_SUBSCRIBE         =0,          // subscription
      ACTION_RENEWAL           =1,          // renewal
      ACTION_SUSPEND           =2,          // suspend
      ACTION_CANCEL            =3,          // cancel
      ACTION_DELETED           =4,          // delete
      //--- enumeration borders
      ACTION_FIRST             =ACTION_SUBSCRIBE,
      ACTION_LAST              =ACTION_DELETED
     };
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTSubscriptionHistory* obj)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- id
   virtual UINT64    ID(void) const=0;
   //--- client login
   virtual UINT64    Login(void) const=0;
   virtual MTAPIRES  Login(const UINT64 login)=0;
   //--- subscription config id
   virtual UINT64    Subscription(void) const=0;
   virtual MTAPIRES  Subscription(const UINT64 subscription_id)=0;
   //--- subscription record id
   virtual UINT64    Record(void) const=0;
   virtual MTAPIRES  Record(const UINT64 record_id)=0;
   //--- EnStatus
   virtual UINT      Action(void) const=0;
   virtual MTAPIRES  Action(const UINT status)=0;
   //--- subscribe time
   virtual INT64     Time(void) const=0;
   virtual MTAPIRES  Time(const INT64 time)=0;
   //--- ammount
   virtual double    Amount(void) const=0;
   virtual MTAPIRES  Amount(const double ammount)=0;
   //--- digits
   virtual UINT      AmountDigits(void) const=0;
   virtual MTAPIRES  AmountDigits(const UINT digits)=0;
   //--- ammount deal
   virtual UINT64    AmountDeal(void) const=0;
   virtual MTAPIRES  AmountDeal(const UINT64 deal)=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTSubscriptionHistory(void) {}
  };

class IMTSubscriptionHistoryArray
  {
public:
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTSubscriptionHistoryArray* array)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- add
   virtual MTAPIRES  Add(IMTSubscriptionHistory* record)=0;
   virtual MTAPIRES  AddCopy(const IMTSubscriptionHistory* record)=0;
   virtual MTAPIRES  Add(IMTSubscriptionHistoryArray* array)=0;
   virtual MTAPIRES  AddCopy(const IMTSubscriptionHistoryArray* array)=0;
   //--- delete & detach
   virtual MTAPIRES  Delete(const UINT pos)=0;
   virtual IMTSubscriptionHistory* Detach(const UINT pos)=0;
   //--- update
   virtual MTAPIRES  Update(const UINT pos,IMTSubscriptionHistory* record)=0;
   virtual MTAPIRES  UpdateCopy(const UINT pos,const IMTSubscriptionHistory* record)=0;
   virtual MTAPIRES  Shift(const UINT pos,const int shift)=0;
   //--- data access
   virtual UINT      Total(void) const=0;
   virtual IMTSubscriptionHistory*  Next(const UINT index) const=0;
   //--- sorting and search
   virtual MTAPIRES  Sort(MTSortFunctionPtr sort_function)=0;
   virtual int       Search(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchGreatOrEq(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchGreater(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchLessOrEq(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchLess(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchLeft(const void *key,MTSortFunctionPtr sort_function) const=0;
   virtual int       SearchRight(const void *key,MTSortFunctionPtr sort_function) const=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTSubscriptionHistoryArray(void) {}
  };

class IMTSubscriptionSink
  {
public:
   //--- events
   virtual void      OnSubscriptionAdd(const IMTSubscription*    /*subscription*/) {  }
   virtual void      OnSubscriptionUpdate(const IMTSubscription* /*subscription*/) {  }
   virtual void      OnSubscriptionDelete(const IMTSubscription* /*subscription*/) {  }
   virtual void      OnSubscriptionJoin(const UINT64 manager,const UINT64 login,const IMTConSubscription* config,IMTSubscription* record,IMTSubscriptionHistory* history) { }
   virtual void      OnSubscriptionCancel(const UINT64 manager,const UINT64 login,const IMTConSubscription* config,IMTSubscription* record,IMTSubscriptionHistory* history) { }
  };

class IMTSubscriptionHistorySink
  {
public:
   //--- events
   virtual void      OnSubscriptionHistoryAdd(const IMTSubscriptionHistory*    /*subscription*/) {  }
   virtual void      OnSubscriptionHistoryUpdate(const IMTSubscriptionHistory* /*subscription*/) {  }
   virtual void      OnSubscriptionHistoryDelete(const IMTSubscriptionHistory* /*subscription*/) {  }
  };

class IMTConVPSGroup
  {
public:
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(IMTConVPSGroup* param)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- group list allowed
   virtual LPCWSTR   Group(void) const=0;
   virtual MTAPIRES  Group(LPCWSTR group)=0;
   //--- minimal balance for sponsored VPS
   virtual double    MinBalance(void) const=0;
   virtual MTAPIRES  MinBalance(const double balance)=0;
   //--- inactivity days to switch off sponsored VPS
   virtual UINT      InactiveDays(void) const=0;
   virtual MTAPIRES  InactiveDays(const UINT days)=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTConVPSGroup(void) {}
  };

class IMTConVPS
  {
public:
   //--- flags
   enum EnFlags
     {
      VPS_NONE          =0x00000000,         // none
      VPS_SPONSOR_ACTIVE=0x00000001,         // activate VPS
     };
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(IMTConVPS* param)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- flags
   virtual UINT64    Flags(void) const=0;
   virtual MTAPIRES  Flags(const UINT64 flags)=0;
   //--- MQL5 login
   virtual LPCWSTR   MQL5Login(void) const=0;
   virtual MTAPIRES  MQL5Login(LPCWSTR login)=0;
   //--- MQL5 password
   virtual LPCWSTR   MQL5Password(void) const=0;
   virtual MTAPIRES  MQL5Password(LPCWSTR password)=0;
   //--- groups settings
   virtual MTAPIRES  GroupAdd(IMTConVPSGroup* group)=0;
   virtual MTAPIRES  GroupUpdate(const UINT pos,const IMTConVPSGroup* group)=0;
   virtual MTAPIRES  GroupDelete(const UINT pos)=0;
   virtual MTAPIRES  GroupClear(void)=0;
   virtual MTAPIRES  GroupShift(const UINT pos,const int shift)=0;
   virtual UINT      GroupTotal(void) const=0;
   virtual MTAPIRES  GroupNext(const UINT pos,IMTConVPSGroup* group) const=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTConVPS(void) {}
  };

class IMTConVPSSink
  {
public:
   virtual void      OnVPSUpdate(const IMTConVPS* /*config*/) {  }
   virtual void      OnVPSSync(void)                          {  }
  };

class IMTConKYCCountry
  {
public:
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTConKYCCountry* country)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- country code
   virtual LPCWSTR   CountryCode(void) const=0;
   virtual MTAPIRES  CountryCode(LPCWSTR code)=0;
  };
//+------------------------------------------------------------------+
//| KYC group config                                                 |
//+------------------------------------------------------------------+
class IMTConKYCGroup
  {
public:
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTConKYCGroup* group)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- phone code
   virtual LPCWSTR   Group(void) const=0;
   virtual MTAPIRES  Group(LPCWSTR group)=0;
  };
//+------------------------------------------------------------------+
//| KYC config                                                       |
//+------------------------------------------------------------------+
class IMTConKYC
  {
public:
   //--- allowed flags
   enum EnFlags
     {
      FLAG_NONE               =0, // none
      FLAG_ENABLED            =1, // config is enabled
      FLAG_DEFAULT            =2, // config is default
      //--- flags borders
      FLAG_FIRST              =FLAG_ENABLED,
      FLAG_ALL                =FLAG_ENABLED|FLAG_DEFAULT
     };
   //--- providers
   enum EnProviderType
     {
      //---- SMS
      PROVIDER_KYC_SUMSUB     =0,
      PROVIDER_KYC_WORLD_CHECK=1,
      PROVIDER_KYC_ESPEAR     =2,
      //--- ranges
      PROVIDER_KYC_FIRST     =PROVIDER_KYC_SUMSUB,
      PROVIDER_KYC_LAST      =PROVIDER_KYC_ESPEAR,
     };
   //--- common methods
   virtual void      Release(void)=0;
   virtual MTAPIRES  Assign(const IMTConKYC* config)=0;
   virtual MTAPIRES  Clear(void)=0;
   //--- name
   virtual LPCWSTR   Name(void) const=0;
   virtual MTAPIRES  Name(LPCWSTR name)=0;
   //--- provider type
   virtual UINT      ProviderType(void) const=0;
   virtual MTAPIRES  ProviderType(const UINT provider)=0;
   //--- provider address
   virtual LPCWSTR   ProviderAddress(void) const=0;
   virtual MTAPIRES  ProviderAddress(LPCWSTR address)=0;
   //--- provider login
   virtual LPCWSTR   ProviderLogin(void) const=0;
   virtual MTAPIRES  ProviderLogin(LPCWSTR login)=0;
   //--- provider password
   virtual LPCWSTR   ProviderPassword(void) const=0;
   virtual MTAPIRES  ProviderPassword(LPCWSTR password)=0;
   //--- provider token
   virtual LPCWSTR   ProviderToken(void) const=0;
   virtual MTAPIRES  ProviderToken(LPCWSTR token)=0;
   //--- EnFlags
   virtual UINT64    Flags(void) const=0;
   virtual MTAPIRES  Flags(const UINT64 flags)=0;
   //--- country settings
   virtual MTAPIRES  CountryAdd(IMTConKYCCountry* country)=0;
   virtual MTAPIRES  CountryUpdate(const UINT pos,const IMTConKYCCountry* country)=0;
   virtual MTAPIRES  CountryDelete(const UINT pos)=0;
   virtual MTAPIRES  CountryClear(void)=0;
   virtual MTAPIRES  CountryShift(const UINT pos,const int shift)=0;
   virtual UINT      CountryTotal(void) const=0;
   virtual MTAPIRES  CountryNext(const UINT pos,IMTConKYCCountry* country) const=0;
   //--- group settings
   virtual MTAPIRES  GroupAdd(IMTConKYCGroup* group)=0;
   virtual MTAPIRES  GroupUpdate(const UINT pos,const IMTConKYCGroup* group)=0;
   virtual MTAPIRES  GroupDelete(const UINT pos)=0;
   virtual MTAPIRES  GroupClear(void)=0;
   virtual MTAPIRES  GroupShift(const UINT pos,const int shift)=0;
   virtual UINT      GroupTotal(void) const=0;
   virtual MTAPIRES  GroupNext(const UINT pos,IMTConKYCGroup* group) const=0;
   //--- explicit destructor is prohibited
protected:
                    ~IMTConKYC(void) {}
  };
//+------------------------------------------------------------------+
//| KYC config events notification interface                         |
//+------------------------------------------------------------------+
class IMTConKYCSink
  {
public:
   virtual void      OnKYCAdd(const IMTConKYC*    /*config*/) {  }
   virtual void      OnKYCUpdate(const IMTConKYC* /*config*/) {  }
   virtual void      OnKYCDelete(const IMTConKYC* /*config*/) {  }
   virtual void      OnKYCSync(void)                          {  }
  };

class IMTServerAPI
  {
public:
   //--- server description
   virtual MTAPIRES  About(MTServerInfo& info)=0;
   //--- license check
   virtual MTAPIRES  LicenseCheck(LPCWSTR license_name)=0;
   //--- memory management
   virtual void*     Allocate(const UINT bytes)=0;
   virtual void      Free(void* ptr)=0;
   //--- server management functions
   virtual MTAPIRES  ServerRestart(void)=0;
   virtual MTAPIRES  ServerSubscribe(IMTServerSink* sink)=0;
   virtual MTAPIRES  ServerUnsubscribe(IMTServerSink* sink)=0;
   virtual MTAPIRES  ServerRestartRemote(const UINT64 id,LPCWSTR reason)=0;
   virtual MTAPIRES  ServerReserved4(void)=0;
   //--- server log functions
   virtual MTAPIRES  LoggerOut(const UINT code,LPCWSTR msg,...)=0;
   virtual MTAPIRES  LoggerRequest(const UINT mode,const UINT type,const INT64 from,const INT64 to,LPCWSTR filter,MTLogRecord*& records,UINT& records_total)=0;
   virtual void      LoggerFlush(void)=0;
   virtual MTAPIRES  LoggerReserved1(void)=0;
   virtual MTAPIRES  LoggerReserved2(void)=0;
   virtual MTAPIRES  LoggerReserved3(void)=0;
   virtual MTAPIRES  LoggerReserved4(void)=0;
   //--- plugins configs
   virtual IMTConPlugin* PluginCreate(void)=0;
   virtual IMTConPluginModule* PluginModuleCreate(void)=0;
   virtual IMTConParam* PluginParamCreate(void)=0;
   virtual MTAPIRES  PluginSubscribe(IMTConPluginSink* sink)=0;
   virtual MTAPIRES  PluginUnsubscribe(IMTConPluginSink* sink)=0;
   virtual MTAPIRES  PluginCurrent(IMTConPlugin* plugin)=0;
   virtual MTAPIRES  PluginAdd(IMTConPlugin* plugin)=0;
   virtual MTAPIRES  PluginDelete(LPCWSTR name)=0;
   virtual MTAPIRES  PluginDelete(const UINT pos)=0;
   virtual MTAPIRES  PluginShift(const UINT pos,const int shift)=0;
   virtual UINT      PluginTotal(void)=0;
   virtual MTAPIRES  PluginNext(const UINT pos,IMTConPlugin* plugin)=0;
   virtual MTAPIRES  PluginGet(LPCWSTR name,IMTConPlugin* plugin)=0;
   virtual UINT      PluginModuleTotal(void)=0;
   virtual MTAPIRES  PluginModuleNext(const UINT pos,IMTConPluginModule* module)=0;
   virtual MTAPIRES  PluginModuleGet(LPCWSTR name,IMTConPluginModule* module)=0;
   virtual MTAPIRES  PluginDelete(const UINT64 server,LPCWSTR name)=0;
   virtual MTAPIRES  PluginGet(const UINT64 server,LPCWSTR name,IMTConPlugin* plugin)=0;
   virtual MTAPIRES  PluginModuleGet(const UINT64 server,LPCWSTR name,IMTConPluginModule* module)=0;
   virtual MTAPIRES  PluginReserved4(void)=0;
   //--- common config 
   virtual IMTConCommon* CommonCreate(void)=0;
   virtual MTAPIRES  CommonSubscribe(IMTConCommonSink* sink)=0;
   virtual MTAPIRES  CommonUnsubscribe(IMTConCommonSink* sink)=0;
   virtual MTAPIRES  CommonGet(IMTConCommon* common)=0;
   virtual MTAPIRES  CommonSet(const IMTConCommon* common)=0;
   virtual MTAPIRES  CommonReserved1(void)=0;
   virtual MTAPIRES  CommonReserved2(void)=0;
   virtual MTAPIRES  CommonReserved3(void)=0;
   virtual MTAPIRES  CommonReserved4(void)=0;
   //--- platform server config
   virtual IMTConServer* NetServerCreate(void)=0;
   virtual IMTConServerRange* NetServerRangeCreate(void)=0;
   virtual MTAPIRES  NetServerSubscribe(IMTConServerSink* sink)=0;
   virtual MTAPIRES  NetServerUnsubscribe(IMTConServerSink* sink)=0;
   virtual MTAPIRES  NetServerAdd(IMTConServer* config)=0;
   virtual MTAPIRES  NetServerDelete(const UINT pos)=0;
   virtual MTAPIRES  NetServerShift(const UINT pos,const int shift)=0;
   virtual UINT      NetServerTotal(void)=0;
   virtual MTAPIRES  NetServerNext(const UINT pos,IMTConServer* config)=0;
   virtual MTAPIRES  NetServerGet(const UINT64 id,IMTConServer* config)=0;
   virtual IMTConAddressRange* NetServerAddressRangeCreate(void)=0;
   virtual IMTConClusterState* NetServerClusterStateCreate(void)=0;
   virtual MTAPIRES  NetServerReserved3(void)=0;
   virtual MTAPIRES  NetServerReserved4(void)=0;
   //--- time config
   virtual IMTConTime* TimeCreate(void)=0;
   virtual MTAPIRES  TimeSubscribe(IMTConTimeSink* sink)=0;
   virtual MTAPIRES  TimeUnsubscribe(IMTConTimeSink* sink)=0;
   virtual INT64     TimeCurrent(void)=0;
   virtual MTAPIRES  TimeGet(IMTConTime* config)=0;
   virtual MTAPIRES  TimeSet(const IMTConTime* config)=0;
   virtual INT64     TimeCurrentMsc(void)=0;
   virtual MTAPIRES  TimeReserved2(void)=0;
   virtual MTAPIRES  TimeReserved3(void)=0;
   virtual MTAPIRES  TimeReserved4(void)=0;
   //--- holidays configuration
   virtual IMTConHoliday* HolidayCreate()=0;
   virtual MTAPIRES  HolidaySubscribe(IMTConHolidaySink* sink)=0;
   virtual MTAPIRES  HolidayUnsubscribe(IMTConHolidaySink* sink)=0;
   virtual MTAPIRES  HolidayAdd(IMTConHoliday* config)=0;
   virtual MTAPIRES  HolidayDelete(const UINT pos)=0;
   virtual MTAPIRES  HolidayShift(const UINT pos,const int shift)=0;
   virtual UINT      HolidayTotal(void)=0;
   virtual MTAPIRES  HolidayNext(const UINT pos,IMTConHoliday* config)=0;
   virtual MTAPIRES  HolidayReserved1(void)=0;
   virtual MTAPIRES  HolidayReserved2(void)=0;
   virtual MTAPIRES  HolidayReserved3(void)=0;
   virtual MTAPIRES  HolidayReserved4(void)=0;
   //--- server firewall configuration
   virtual IMTConFirewall* FirewallCreate()=0;
   virtual MTAPIRES  FirewallSubscribe(IMTConFirewallSink* sink)=0;
   virtual MTAPIRES  FirewallUnsubscribe(IMTConFirewallSink* sink)=0;
   virtual MTAPIRES  FirewallAdd(IMTConFirewall* config)=0;
   virtual MTAPIRES  FirewallDelete(const UINT pos)=0;
   virtual MTAPIRES  FirewallShift(const UINT pos,const int shift)=0;
   virtual UINT      FirewallTotal(void)=0;
   virtual MTAPIRES  FirewallNext(const UINT pos,IMTConFirewall* config)=0;
   virtual MTAPIRES  FirewallReserved1(void)=0;
   virtual MTAPIRES  FirewallReserved2(void)=0;
   virtual MTAPIRES  FirewallReserved3(void)=0;
   virtual MTAPIRES  FirewallReserved4(void)=0;
   //--- symbols configuration
   virtual IMTConSymbol* SymbolCreate(void)=0;
   virtual IMTConSymbolSession* SymbolSessionCreate(void)=0;
   virtual MTAPIRES  SymbolSubscribe(IMTConSymbolSink* sink)=0;
   virtual MTAPIRES  SymbolUnsubscribe(IMTConSymbolSink* sink)=0;
   virtual MTAPIRES  SymbolAdd(IMTConSymbol* symbol)=0;
   virtual MTAPIRES  SymbolDelete(LPCWSTR name)=0;
   virtual MTAPIRES  SymbolDelete(const UINT pos)=0;
   virtual MTAPIRES  SymbolShift(const UINT pos,const int shift)=0;
   virtual UINT      SymbolTotal(void)=0;
   virtual MTAPIRES  SymbolNext(const UINT pos,IMTConSymbol* symbol)=0;
   virtual MTAPIRES  SymbolGet(LPCWSTR name,IMTConSymbol* symbol)=0;
   virtual MTAPIRES  SymbolGet(LPCWSTR name,const IMTConGroup* group,IMTConSymbol* symbol)=0;
   virtual MTAPIRES  SymbolExist(const IMTConSymbol* symbol,const IMTConGroup* group)=0;
   virtual MTAPIRES  SymbolReserved1(void)=0;
   virtual MTAPIRES  SymbolReserved2(void)=0;
   virtual MTAPIRES  SymbolReserved3(void)=0;
   virtual MTAPIRES  SymbolReserved4(void)=0;
   //--- clients group configuration
   virtual IMTConGroup* GroupCreate(void)=0;
   virtual IMTConGroupSymbol* GroupSymbolCreate(void)=0;
   virtual IMTConCommission* GroupCommissionCreate(void)=0;
   virtual IMTConCommTier* GroupTierCreate(void)=0;
   virtual MTAPIRES  GroupSubscribe(IMTConGroupSink* sink)=0;
   virtual MTAPIRES  GroupUnsubscribe(IMTConGroupSink* sink)=0;
   virtual MTAPIRES  GroupAdd(IMTConGroup* group)=0;
   virtual MTAPIRES  GroupDelete(LPCWSTR name)=0;
   virtual MTAPIRES  GroupDelete(const UINT pos)=0;
   virtual MTAPIRES  GroupShift(const UINT pos,const int shift)=0;
   virtual UINT      GroupTotal(void)=0;
   virtual MTAPIRES  GroupNext(const UINT pos,IMTConGroup* group)=0;
   virtual MTAPIRES  GroupGet(LPCWSTR name,IMTConGroup* group)=0;
   virtual MTAPIRES  GroupReserved1(void)=0;
   virtual MTAPIRES  GroupReserved2(void)=0;
   virtual MTAPIRES  GroupReserved3(void)=0;
   virtual MTAPIRES  GroupReserved4(void)=0;
   //--- managers configuration
   virtual IMTConManager* ManagerCreate(void)=0;
   virtual IMTConManagerAccess* ManagerAccessCreate(void)=0;
   virtual MTAPIRES  ManagerSubscribe(IMTConManagerSink* sink)=0;
   virtual MTAPIRES  ManagerUnsubscribe(IMTConManagerSink* sink)=0;
   virtual MTAPIRES  ManagerAdd(IMTConManager* manager)=0;
   virtual MTAPIRES  ManagerDelete(const UINT pos)=0;
   virtual MTAPIRES  ManagerShift(const UINT pos,const int shift)=0;
   virtual UINT      ManagerTotal(void)=0;
   virtual MTAPIRES  ManagerNext(const UINT pos,IMTConManager* manager)=0;
   virtual MTAPIRES  ManagerGet(const UINT64 login,IMTConManager* manager)=0;
   virtual MTAPIRES  ManagerReserved1(void)=0;
   virtual MTAPIRES  ManagerReserved2(void)=0;
   virtual MTAPIRES  ManagerReserved3(void)=0;
   virtual MTAPIRES  ManagerReserved4(void)=0;
   //--- history synchronization configuration
   virtual IMTConHistorySync* HistorySyncCreate(void)=0;
   virtual MTAPIRES  HistorySyncSubscribe(IMTConHistorySyncSink* sink)=0;
   virtual MTAPIRES  HistorySyncUnsubscribe(IMTConHistorySyncSink* sink)=0;
   virtual MTAPIRES  HistorySyncAdd(IMTConHistorySync* config)=0;
   virtual MTAPIRES  HistorySyncDelete(const UINT pos)=0;
   virtual MTAPIRES  HistorySyncShift(const UINT pos,const int shift)=0;
   virtual UINT      HistorySyncTotal(void)=0;
   virtual MTAPIRES  HistorySyncNext(const UINT pos,IMTConHistorySync* config)=0;
   virtual MTAPIRES  HistorySyncReserved1(void)=0;
   virtual MTAPIRES  HistorySyncReserved2(void)=0;
   virtual MTAPIRES  HistorySyncReserved3(void)=0;
   virtual MTAPIRES  HistorySyncReserved4(void)=0;
   //--- datafeeds configuration
   virtual IMTConFeeder* FeederCreate(void)=0;
   virtual IMTConFeederModule* FeederModuleCreate(void)=0;
   virtual IMTConParam* FeederParamCreate(void)=0;
   virtual IMTConFeederTranslate* FeederTranslateCreate(void)=0;
   virtual MTAPIRES  FeederSubscribe(IMTConFeederSink* sink)=0;
   virtual MTAPIRES  FeederUnsubscribe(IMTConFeederSink* sink)=0;
   virtual MTAPIRES  FeederAdd(IMTConFeeder* feeder)=0;
   virtual MTAPIRES  FeederDelete(LPCWSTR name)=0;
   virtual MTAPIRES  FeederDelete(const UINT pos)=0;
   virtual MTAPIRES  FeederShift(const UINT pos,const int shift)=0;
   virtual UINT      FeederTotal(void)=0;
   virtual MTAPIRES  FeederNext(const UINT pos,IMTConFeeder* feeder)=0;
   virtual MTAPIRES  FeederGet(LPCWSTR name,IMTConFeeder* feeder)=0;
   virtual UINT      FeederModuleTotal(void)=0;
   virtual MTAPIRES  FeederModuleNext(const UINT pos,IMTConFeederModule* module)=0;
   virtual MTAPIRES  FeederModuleGet(LPCWSTR name,IMTConFeederModule* module)=0;
   virtual MTAPIRES  FeederRestart(LPCWSTR name)=0;
   virtual MTAPIRES  FeederReserved2(void)=0;
   virtual MTAPIRES  FeederReserved3(void)=0;
   virtual MTAPIRES  FeederReserved4(void)=0;
   //--- gateways configuration
   virtual IMTConGateway* GatewayCreate(void)=0;
   virtual IMTConGatewayModule* GatewayModuleCreate(void)=0;
   virtual IMTConParam* GatewayParamCreate(void)=0;
   virtual IMTConGatewayTranslate* GatewayTranslateCreate(void)=0;
   virtual MTAPIRES  GatewaySubscribe(IMTConGatewaySink* sink)=0;
   virtual MTAPIRES  GatewayUnsubscribe(IMTConGatewaySink* sink)=0;
   virtual MTAPIRES  GatewayAdd(IMTConGateway* gateway)=0;
   virtual MTAPIRES  GatewayDelete(LPCWSTR name)=0;
   virtual MTAPIRES  GatewayDelete(const UINT pos)=0;
   virtual MTAPIRES  GatewayShift(const UINT pos,const int shift)=0;
   virtual UINT      GatewayTotal(void)=0;
   virtual MTAPIRES  GatewayNext(const UINT pos,IMTConGateway* gateway)=0;
   virtual MTAPIRES  GatewayGet(LPCWSTR name,IMTConGateway* gateway)=0;
   virtual UINT      GatewayModuleTotal(void)=0;
   virtual MTAPIRES  GatewayModuleNext(const UINT pos,IMTConGatewayModule* module)=0;
   virtual MTAPIRES  GatewayModuleGet(LPCWSTR name,IMTConGatewayModule* module)=0;
   virtual MTAPIRES  GatewayRestart(LPCWSTR name)=0;
   virtual MTAPIRES  GatewayReserved2(void)=0;
   virtual MTAPIRES  GatewayReserved3(void)=0;
   virtual MTAPIRES  GatewayReserved4(void)=0;
   //--- report configuration
   virtual IMTConReport* ReportCreate(void)=0;
   virtual IMTConReportModule* ReportModuleCreate(void)=0;
   virtual IMTConParam* ReportParamCreate(void)=0;
   virtual MTAPIRES  ReportSubscribe(IMTConReportSink* sink)=0;
   virtual MTAPIRES  ReportUnsubscribe(IMTConReportSink* sink)=0;
   virtual MTAPIRES  ReportAdd(IMTConReport* report)=0;
   virtual MTAPIRES  ReportDelete(LPCWSTR name)=0;
   virtual MTAPIRES  ReportDelete(const UINT pos)=0;
   virtual MTAPIRES  ReportShift(const UINT pos,const int shift)=0;
   virtual UINT      ReportTotal(void)=0;
   virtual MTAPIRES  ReportNext(const UINT pos,IMTConReport* report)=0;
   virtual MTAPIRES  ReportGet(LPCWSTR name,IMTConReport* report)=0;
   virtual UINT      ReportModuleTotal(void)=0;
   virtual MTAPIRES  ReportModuleNext(const UINT pos,IMTConReportModule* module)=0;
   virtual MTAPIRES  ReportModuleGet(LPCWSTR name,IMTConReportModule* module)=0;
   virtual MTAPIRES  ReportDelete(const UINT64 server,LPCWSTR name)=0;
   virtual MTAPIRES  ReportGet(const UINT64 server,LPCWSTR name,IMTConReport* report)=0;
   virtual MTAPIRES  ReportModuleGet(const UINT64 server,LPCWSTR name,IMTConReportModule* module)=0;
   virtual MTAPIRES  ReportReserved4(void)=0;
   //--- routing configuration
   virtual IMTConRoute* RouteCreate(void)=0;
   virtual IMTConCondition* RouteConditionCreate(void)=0;
   virtual IMTConRouteDealer* RouteDealerCreate(void)=0;
   virtual MTAPIRES  RouteSubscribe(IMTConRouteSink* sink)=0;
   virtual MTAPIRES  RouteUnsubscribe(IMTConRouteSink* sink)=0;
   virtual MTAPIRES  RouteAdd(IMTConRoute* route)=0;
   virtual MTAPIRES  RouteDelete(LPCWSTR name)=0;
   virtual MTAPIRES  RouteDelete(const UINT pos)=0;
   virtual MTAPIRES  RouteShift(const UINT pos,const int shift)=0;
   virtual UINT      RouteTotal(void)=0;
   virtual MTAPIRES  RouteNext(const UINT pos,IMTConRoute* route)=0;
   virtual MTAPIRES  RouteGet(LPCWSTR name,IMTConRoute* route)=0;
   virtual MTAPIRES  RouteReserved1(void)=0;
   virtual MTAPIRES  RouteReserved2(void)=0;
   virtual MTAPIRES  RouteReserved3(void)=0;
   virtual MTAPIRES  RouteReserved4(void)=0;
   //--- clients database
   virtual IMTUser*  UserCreate(void)=0;
   virtual IMTAccount* UserCreateAccount(void)=0;
   virtual MTAPIRES  UserSubscribe(IMTUserSink* sink)=0;
   virtual MTAPIRES  UserUnsubscribe(IMTUserSink* sink)=0;
   virtual MTAPIRES  UserAdd(IMTUser* user,LPCWSTR master_pass,LPCWSTR investor_pass)=0;
   virtual MTAPIRES  UserDelete(const UINT64 login)=0;
   virtual MTAPIRES  UserUpdate(IMTUser* user)=0;
   virtual UINT      UserTotal(void)=0;
   virtual MTAPIRES  UserGet(const UINT64 login,IMTUser* user)=0;
   virtual MTAPIRES  UserGroup(const UINT64 login,MTAPISTR& group)=0;
   virtual MTAPIRES  UserLogins(LPCWSTR group,UINT64*& logins,UINT& logins_total)=0;
   virtual MTAPIRES  UserPasswordCheck(const UINT type,const UINT64 login,LPCWSTR password)=0;
   virtual MTAPIRES  UserPasswordChange(const UINT type,const UINT64 login,LPCWSTR password)=0;
   virtual MTAPIRES  UserCertDelete(const UINT64 login)=0;
   virtual MTAPIRES  UserCertConfirm(const UINT64 login)=0;
   virtual MTAPIRES  UserDepositChangeRaw(const UINT64 login,const double value,const UINT type,LPCWSTR comment,UINT64& deal_id)=0;
   virtual MTAPIRES  UserDepositChange(const UINT64 login,const double value,const UINT action,LPCWSTR comment,UINT64& deal_id)=0;
   virtual MTAPIRES  UserAccountGet(const UINT64 login,IMTAccount* account)=0;
   virtual MTAPIRES  UserArchive(const UINT64 login)=0;
   virtual MTAPIRES  UserArchiveGet(const UINT64 login,IMTUser* user)=0;
   virtual MTAPIRES  UserRestore(IMTUser* user)=0;
   virtual MTAPIRES  UserArchiveLogins(LPCWSTR group,UINT64*& logins,UINT& logins_total)=0;
   //--- deals database
   virtual IMTDeal*  DealCreate(void)=0;
   virtual IMTDealArray* DealCreateArray(void)=0;
   virtual MTAPIRES  DealSubscribe(IMTDealSink* sink)=0;
   virtual MTAPIRES  DealUnsubscribe(IMTDealSink* sink)=0;
   virtual MTAPIRES  DealDelete(const UINT64 ticket)=0;
   virtual MTAPIRES  DealUpdate(IMTDeal* deal)=0;
   virtual MTAPIRES  DealGet(const UINT64 ticket,IMTDeal* deal)=0;
   virtual MTAPIRES  DealGet(const INT64 from,const INT64 to,const UINT64 login,IMTDealArray* deals)=0;
   virtual MTAPIRES  DealAdd(IMTDeal* deal)=0;
   virtual MTAPIRES  DealPerform(IMTDeal* deal)=0;
   virtual MTAPIRES  DealPerformCloseBy(IMTDeal* deal,IMTDeal* dealby)=0;
   virtual MTAPIRES  DealDeleteBatch(const UINT64* tickets,const UINT tickets_total,MTAPIRES* results)=0;
   //--- trade positions database
   virtual IMTPosition* PositionCreate(void)=0;
   virtual IMTPositionArray* PositionCreateArray(void)=0;
   virtual MTAPIRES  PositionSubscribe(IMTPositionSink* sink)=0;
   virtual MTAPIRES  PositionUnsubscribe(IMTPositionSink* sink)=0;
   virtual MTAPIRES  PositionDelete(const UINT64 login,LPCWSTR symbol)=0;
   virtual MTAPIRES  PositionUpdate(IMTPosition* position)=0;
   virtual MTAPIRES  PositionGet(const UINT64 login,LPCWSTR symbol,IMTPosition* position)=0;
   virtual MTAPIRES  PositionGet(const UINT64 login,IMTPositionArray* position)=0;
   virtual MTAPIRES  PositionDeleteByTicket(const UINT64 ticket)=0;
   virtual MTAPIRES  PositionGetByTicket(const UINT64 ticket,IMTPosition* position)=0;
   virtual MTAPIRES  PositionCheck(const UINT64 login,IMTPositionArray* current,IMTPositionArray* invalid,IMTPositionArray* missed,IMTPositionArray* nonexist)=0;
   virtual MTAPIRES  PositionFix(const UINT64 login,IMTPositionArray* current)=0;
   //--- open orders database
   virtual IMTOrder* OrderCreate(void)=0;
   virtual IMTOrderArray* OrderCreateArray(void)=0;
   virtual MTAPIRES  OrderSubscribe(IMTOrderSink* sink)=0;
   virtual MTAPIRES  OrderUnsubscribe(IMTOrderSink* sink)=0;
   virtual MTAPIRES  OrderDelete(const UINT64 ticket)=0;
   virtual MTAPIRES  OrderUpdate(IMTOrder* order)=0;
   virtual MTAPIRES  OrderGet(const UINT64 ticket,IMTOrder* order)=0;
   virtual MTAPIRES  OrderGet(const UINT64 login,IMTOrderArray* orders)=0;
   virtual MTAPIRES  OrderAdd(IMTOrder* order)=0;
   virtual MTAPIRES  OrderDeleteBatch(const UINT64* tickets,const UINT tickets_total,MTAPIRES* results)=0;
   virtual MTAPIRES  OrderUpdateBatch(IMTOrderArray* orders,MTAPIRES* results)=0;
   virtual MTAPIRES  OrderUpdateBatchArray(IMTOrder** orders,const UINT orders_total,MTAPIRES* results)=0;
   //--- orders history database
   virtual MTAPIRES  HistorySubscribe(IMTHistorySink* sink)=0;
   virtual MTAPIRES  HistoryUnsubscribe(IMTHistorySink* sink)=0;
   virtual MTAPIRES  HistoryDelete(const UINT64 ticket)=0;
   virtual MTAPIRES  HistoryUpdate(IMTOrder* order)=0;
   virtual MTAPIRES  HistoryGet(const UINT64 ticket,IMTOrder* order)=0;
   virtual MTAPIRES  HistoryGet(const INT64 from,const INT64 to,const UINT64 login,IMTOrderArray* orders)=0;
   virtual MTAPIRES  HistoryReopen(const UINT64 ticket)=0;
   virtual MTAPIRES  HistoryAdd(IMTOrder* order)=0;
   virtual MTAPIRES  HistoryDeleteBatch(const UINT64* tickets,const UINT tickets_total,MTAPIRES* results)=0;
   virtual MTAPIRES  HistoryUpdateBatch(IMTOrderArray* orders,MTAPIRES* results)=0;
   //--- daily reports database
   virtual IMTDaily* DailyCreate(void)=0;
   virtual IMTDailyArray* DailyCreateArray(void)=0;
   virtual MTAPIRES  DailySubscribe(IMTDailySink* sink)=0;
   virtual MTAPIRES  DailyUnsubscribe(IMTDailySink* sink)=0;
   virtual MTAPIRES  DailyGet(const INT64 from,const INT64 to,const UINT64 login,IMTDailyArray* daily)=0;
   virtual MTAPIRES  DailyGetLight(const INT64 from,const INT64 to,const UINT64 login,IMTDailyArray* daily)=0;
   virtual MTAPIRES  DailySelectByGroup(LPCWSTR group,INT64 from,INT64 to,const IMTDatasetRequest *request,IMTDataset *dataset)=0;
   virtual MTAPIRES  DailySelectByLogins(const UINT64 *logins,UINT logins_total,INT64 from,INT64 to,const IMTDatasetRequest *request,IMTDataset *dataset)=0;
   virtual MTAPIRES  DailyReserved4(void)=0;
   //--- ticks data
   virtual MTAPIRES  TickSubscribe(IMTTickSink* sink)=0;
   virtual MTAPIRES  TickUnsubscribe(IMTTickSink* sink)=0;
   virtual MTAPIRES  TickAdd(MTTick& tick)=0;
   virtual MTAPIRES  TickAddStat(MTTick& tick,MTTickStat& stat)=0;
   virtual MTAPIRES  TickLast(LPCWSTR symbol,MTTickShort& tick)=0;
   virtual MTAPIRES  TickLast(const IMTConSymbol* symbol,MTTickShort& tick)=0;
   virtual MTAPIRES  TickStat(LPCWSTR symbol,MTTickStat& stat)=0;
   virtual MTAPIRES  TickGet(LPCWSTR symbol,const INT64 from,const INT64 to,MTTickShort*& ticks,UINT& ticks_total)=0;
   virtual MTAPIRES  TickGet(const IMTConSymbol* symbol,const INT64 from,const INT64 to,MTTickShort*& ticks,UINT& ticks_total)=0;
   virtual MTAPIRES  TickHistoryGetRaw(LPCWSTR symbol,const INT64 from,const INT64 to,MTTickShort*& ticks,UINT& ticks_total)=0;
   virtual MTAPIRES  TickHistoryGet(LPCWSTR symbol,const INT64 from,const INT64 to,MTTickShort*& ticks,UINT& ticks_total)=0;
   virtual MTAPIRES  TickAddBatch(MTTick* ticks,const UINT ticks_total)=0;
   virtual MTAPIRES  TickReserved3(void)=0;
   virtual MTAPIRES  TickReserved4(void)=0;
   //--- internal mail
   virtual IMTMail*  MailCreate(void)=0;
   virtual MTAPIRES  MailSubscribe(IMTMailSink* sink)=0;
   virtual MTAPIRES  MailUnsubscribe(IMTMailSink* sink)=0;
   virtual MTAPIRES  MailSend(IMTMail* mail)=0;
   virtual MTAPIRES  MailReserved1(void)=0;
   virtual MTAPIRES  MailReserved2(void)=0;
   virtual MTAPIRES  MailReserved3(void)=0;
   virtual MTAPIRES  MailReserved4(void)=0;
   //--- internal news
   virtual IMTNews*  NewsCreate(void)=0;
   virtual MTAPIRES  NewsSubscribe(IMTNewsSink* sink)=0;
   virtual MTAPIRES  NewsUnsubscribe(IMTNewsSink* sink)=0;
   virtual MTAPIRES  NewsSend(IMTNews* news)=0;
   virtual MTAPIRES  NewsReserved1(void)=0;
   virtual MTAPIRES  NewsReserved2(void)=0;
   virtual MTAPIRES  NewsReserved3(void)=0;
   virtual MTAPIRES  NewsReserved4(void)=0;
   //--- custom commands processing
   virtual MTAPIRES  CustomSubscribe(IMTCustomSink* sink)=0;
   virtual MTAPIRES  CustomUnsubscribe(IMTCustomSink* sink)=0;
   virtual IMTByteStream* CustomCreateStream(void)=0;
   virtual MTAPIRES  CustomReserved2(void)=0;
   virtual MTAPIRES  CustomReserved3(void)=0;
   virtual MTAPIRES  CustomReserved4(void)=0;
   //--- trade methods
   virtual IMTRequest* TradeRequestCreate(void)=0;
   virtual MTAPIRES  TradeSubscribe(IMTTradeSink* sink)=0;
   virtual MTAPIRES  TradeUnsubscribe(IMTTradeSink* sink)=0;
   virtual MTAPIRES  TradeRequest(IMTRequest* request)=0;
   virtual MTAPIRES  TradeProfit(LPCWSTR group,LPCWSTR symbol,const UINT type,const UINT64 volume,const double price_open,const double price_close,double& profit,double& profit_rate)=0;
   virtual MTAPIRES  TradeRateBuy(LPCWSTR base,LPCWSTR currency,double& rate,LPCWSTR group=NULL,LPCWSTR symbol=NULL,const double price=0)=0;
   virtual MTAPIRES  TradeRateSell(LPCWSTR base,LPCWSTR currency,double& rate,LPCWSTR group=NULL,LPCWSTR symbol=NULL,const double price=0)=0;
   virtual MTAPIRES  TradeMarginCheck(const UINT64 login,LPCWSTR symbol,const UINT type,const UINT64 volume,const double price,IMTAccount* account_new=NULL,IMTAccount* account_current=NULL)=0;
   virtual MTAPIRES  TradeMarginCheck(const IMTOrder* order,IMTAccount* account_new=NULL,IMTAccount* account_current=NULL)=0;
   virtual MTAPIRES  TradeBalanceCheckObsolete(const UINT64 login,const UINT fixflag,double& balance_user,double& balance_history)=0;
   virtual MTAPIRES  TradeSubscribeEOD(IMTEndOfDaySink* sink)=0;
   virtual MTAPIRES  TradeUnsubscribeEOD(IMTEndOfDaySink* sink)=0;
   virtual MTAPIRES  TradeBalanceCheck(const UINT64 login,const UINT fixflag,double& balance_user,double& balance_history,double& credit_user,double& credit_history)=0;
   virtual MTAPIRES  TradeAccountSet(const IMTUser *user,const IMTAccount *account,const IMTOrderArray *orders,const IMTPositionArray *positions)=0;
   virtual IMTConfirm* TradeConfirmCreate(void)=0;
   virtual IMTExecution* TradeExecutionCreate(void)=0;
   virtual IMTRequestArray* TradeRequestCreateArray(void)=0;
   virtual MTAPIRES  TradeProfitExt(LPCWSTR group,LPCWSTR symbol,const UINT type,const UINT64 volume,const double price_open,const double price_close,double& profit,double& profit_rate)=0;
   //--- books access
   virtual MTAPIRES  BookSubscribe(IMTBookSink* sink)=0;
   virtual MTAPIRES  BookUnsubscribe(IMTBookSink* sink)=0;
   virtual MTAPIRES  BookGet(LPCWSTR symbol,MTBook& book)=0;
   virtual MTAPIRES  BookReserved1(void)=0;
   virtual MTAPIRES  BookReserved2(void)=0;
   virtual MTAPIRES  BookReserved3(void)=0;
   virtual MTAPIRES  BookReserved4(void)=0;
   //--- chart methods
   virtual MTAPIRES  ChartGet(LPCWSTR symbol,const INT64 from,const INT64 to,MTChartBar*& bars,UINT& bars_total)=0;
   virtual MTAPIRES  ChartDelete(LPCWSTR symbol,const INT64* bars_dates,const UINT bars_dates_total)=0;
   virtual MTAPIRES  ChartUpdate(LPCWSTR symbol,const MTChartBar* bars,const UINT bars_total)=0;
   virtual MTAPIRES  ChartSplit(LPCWSTR symbol,const UINT new_shares,const UINT old_shares,const UINT rounding_rule,const INT64 datetime_from,const INT64 datetime_to)=0;
   virtual MTAPIRES  ChartReserved2(void)=0;
   virtual MTAPIRES  ChartReserved3(void)=0;
   virtual MTAPIRES  ChartReserved4(void)=0;
   //--- clients database
   virtual IMTCertificate *UserCertCreate(void)=0;
   virtual MTAPIRES  UserCertUpdate(const UINT64 login,const IMTCertificate *certificate)=0;
   virtual MTAPIRES  UserCertGet(const UINT64 login,IMTCertificate *certificate)=0;
   virtual MTAPIRES  UserCertReserved1(void)=0;
   virtual MTAPIRES  UserCertReserved2(void)=0;
   virtual MTAPIRES  UserCertReserved3(void)=0;
   virtual MTAPIRES  UserCertReserved4(void)=0;
   //--- spreads configuration
   virtual IMTConSpread* SpreadCreate(void)=0;
   virtual IMTConSpreadLeg* SpreadLegCreate(void)=0;
   virtual MTAPIRES  SpreadSubscribe(IMTConSpreadSink* sink)=0;
   virtual MTAPIRES  SpreadUnsubscribe(IMTConSpreadSink* sink)=0;
   virtual MTAPIRES  SpreadAdd(IMTConSpread* spread)=0;
   virtual MTAPIRES  SpreadDelete(const UINT pos)=0;
   virtual MTAPIRES  SpreadShift(const UINT pos,const int shift)=0;
   virtual UINT      SpreadTotal(void)=0;
   virtual MTAPIRES  SpreadNext(const UINT pos,IMTConSpread* spread)=0;
   virtual MTAPIRES  SpreadGet(const UINT id,IMTConSpread* spread)=0;
   virtual MTAPIRES  SpreadReserved1(void)=0;
   virtual MTAPIRES  SpreadReserved2(void)=0;
   virtual MTAPIRES  SpreadReserved3(void)=0;
   virtual MTAPIRES  SpreadReserved4(void)=0;
   //--- online connection data
   virtual IMTOnline* OnlineCreate(void)=0;
   virtual IMTOnlineArray* OnlineCreateArray(void)=0;
   virtual UINT      OnlineTotal(void)=0;
   virtual MTAPIRES  OnlineNext(const UINT pos,IMTOnline* online)=0;
   virtual MTAPIRES  OnlineGet(const UINT64 login,IMTOnlineArray* online)=0;
   virtual MTAPIRES  OnlineDisconnect(IMTOnline* online)=0;
   virtual MTAPIRES  OnlineDisconnectBatch(IMTOnlineArray* online,MTAPIRES* results)=0;
   virtual MTAPIRES  OnlineDisconnectBatchArray(IMTOnline** online,const UINT online_total,MTAPIRES* results)=0;
   virtual MTAPIRES  OnlineReserved4(void)=0;
   //--- notifications
   virtual MTAPIRES  NotificationsSend(LPCWSTR metaquotes_ids,LPCWSTR message)=0;
   virtual MTAPIRES  NotificationsSend(const UINT64* logins,const UINT logins_total,LPCWSTR message)=0;
   virtual MTAPIRES  NotificationsReserved1(void)=0;
   virtual MTAPIRES  NotificationsReserved2(void)=0;
   virtual MTAPIRES  NotificationsReserved3(void)=0;
   virtual MTAPIRES  NotificationsReserved4(void)=0;
   //--- deals
   virtual MTAPIRES  DealUpdateBatch(IMTDealArray* deals,MTAPIRES* results)=0;
   virtual MTAPIRES  DealUpdateBatchArray(IMTDeal** deals,const UINT deals_total,MTAPIRES* results)=0;
   virtual MTAPIRES  DealAddBatch(IMTDealArray* deals,MTAPIRES* results)=0;
   virtual MTAPIRES  DealAddBatchArray(IMTDeal** deals,const UINT deals_total,MTAPIRES* results)=0;
   virtual MTAPIRES  DealPerformBatch(IMTDealArray* deals,MTAPIRES* results)=0;
   virtual MTAPIRES  DealPerformBatchArray(IMTDeal** deals,const UINT deals_total,MTAPIRES* results)=0;
   virtual MTAPIRES  DealSelectByGroup(LPCWSTR group,INT64 from,INT64 to,const IMTDatasetRequest *request,IMTDataset *dataset)=0;
   virtual MTAPIRES  DealSelectByLogins(const UINT64 *logins,UINT logins_total,INT64 from,INT64 to,const IMTDatasetRequest *request,IMTDataset *dataset)=0;
   virtual MTAPIRES  DealGetByGroup(LPCWSTR group,INT64 from,INT64 to,IMTDealArray *deals)=0;
   virtual MTAPIRES  DealGetByLogins(const UINT64 *logins,UINT logins_total,INT64 from,INT64 to,IMTDealArray *deals)=0;
   //--- orders
   virtual MTAPIRES  OrderAddBatch(IMTOrderArray* orders,MTAPIRES* results)=0;
   virtual MTAPIRES  OrderAddBatchArray(IMTOrder** orders,const UINT orders_total,MTAPIRES* results)=0;
   virtual MTAPIRES  OrderSelectByGroup(LPCWSTR group,const IMTDatasetRequest *request,IMTDataset *dataset)=0;
   virtual MTAPIRES  OrderSelectByLogins(const UINT64 *logins,UINT logins_total,const IMTDatasetRequest *request,IMTDataset *dataset)=0;
   virtual MTAPIRES  OrderGetByGroup(LPCWSTR group,IMTOrderArray *orders)=0;
   virtual MTAPIRES  OrderGetByLogins(const UINT64 *logins,UINT logins_total,IMTOrderArray *orders)=0;
   //--- history
   virtual MTAPIRES  HistoryUpdateBatchArray(IMTOrder** orders,const UINT orders_total,MTAPIRES* results)=0;
   virtual MTAPIRES  HistoryAddBatch(IMTOrderArray* orders,MTAPIRES* results)=0;
   virtual MTAPIRES  HistoryAddBatchArray(IMTOrder** orders,const UINT orders_total,MTAPIRES* results)=0;
   virtual MTAPIRES  HistorySelectByGroup(LPCWSTR group,INT64 from,INT64 to,const IMTDatasetRequest *request,IMTDataset *dataset)=0;
   virtual MTAPIRES  HistorySelectByLogins(const UINT64 *logins,UINT logins_total,INT64 from,INT64 to,const IMTDatasetRequest *request,IMTDataset *dataset)=0;
   virtual MTAPIRES  HistoryGetByGroup(LPCWSTR group,INT64 from,INT64 to,IMTOrderArray *orders)=0;
   virtual MTAPIRES  HistoryGetByLogins(const UINT64 *logins,UINT logins_total,INT64 from,INT64 to,IMTOrderArray *orders)=0;
   //--- dealing
   virtual MTAPIRES  DealerStart(const UINT64 dealer,IMTRequestSink* sink)=0;
   virtual MTAPIRES  DealerStop(const UINT64 dealer,IMTRequestSink* sink)=0;
   virtual MTAPIRES  DealerGet(const UINT64 dealer,IMTRequest* request)=0;
   virtual MTAPIRES  DealerLock(const UINT64 dealer,const UINT id,IMTRequest* request)=0;
   virtual MTAPIRES  DealerAnswer(const UINT64 dealer,IMTConfirm* confirm)=0;
   virtual UINT      DealerRequestTotal(const UINT64 dealer)=0;
   virtual MTAPIRES  DealerRequestNext(const UINT64 dealer,const UINT pos,IMTRequest* request)=0;
   virtual MTAPIRES  DealerRequestGet(const UINT64 dealer,const UINT id,IMTRequest* request)=0;
   virtual MTAPIRES  DealerRequestGetAll(const UINT64 dealer,IMTRequestArray* requests)=0;
   virtual MTAPIRES  DealerExecution(LPCWSTR gateway_name,LPCWSTR gateway_type,IMTExecution *execution)=0;
   virtual MTAPIRES  DealerReserved2(void)=0;
   virtual MTAPIRES  DealerReserved3(void)=0;
   virtual MTAPIRES  DealerReserved4(void)=0;
   //--- trade methods
   virtual MTAPIRES  TradeMarginCheckExt(const UINT64 login,LPCWSTR symbol,const UINT type,const UINT64 volume,const double price,IMTAccount* account_new=NULL,IMTAccount* account_current=NULL)=0;
   virtual MTAPIRES  TradeReserved1(void)=0;
   virtual MTAPIRES  TradeReserved2(void)=0;
   virtual MTAPIRES  TradeReserved3(void)=0;
   virtual MTAPIRES  TradeReserved4(void)=0;
   virtual MTAPIRES  TradeReserved5(void)=0;
   virtual MTAPIRES  TradeReserved6(void)=0;
   //--- email servers configuration
   virtual IMTConEmail* EmailCreate()=0;
   virtual MTAPIRES  EmailSubscribe(IMTConEmailSink* sink)=0;
   virtual MTAPIRES  EmailUnsubscribe(IMTConEmailSink* sink)=0;
   virtual MTAPIRES  EmailAdd(IMTConEmail* config)=0;
   virtual MTAPIRES  EmailDelete(LPCWSTR name)=0;
   virtual MTAPIRES  EmailDelete(const UINT pos)=0;
   virtual MTAPIRES  EmailShift(const UINT pos,const int shift)=0;
   virtual UINT      EmailTotal(void)=0;
   virtual MTAPIRES  EmailNext(const UINT pos,IMTConEmail* email)=0;
   virtual MTAPIRES  EmailGet(LPCWSTR name,IMTConEmail* email)=0;
   virtual MTAPIRES  EmailSend(LPCWSTR account,LPCWSTR to,LPCWSTR to_name,LPCWSTR subject,LPCWSTR body)=0;
   virtual MTAPIRES  EmailReserved2(void)=0;
   virtual MTAPIRES  EmailReserved3(void)=0;
   virtual MTAPIRES  EmailReserved4(void)=0;
   //--- messengers configuration
   virtual IMTConMessenger* MessengerCreate()=0;
   virtual MTAPIRES  MessengerSubscribe(IMTConMessengerSink* sink)=0;
   virtual MTAPIRES  MessengerUnsubscribe(IMTConMessengerSink* sink)=0;
   virtual MTAPIRES  MessengerAdd(IMTConMessenger* config)=0;
   virtual MTAPIRES  MessengerDelete(LPCWSTR name)=0;
   virtual MTAPIRES  MessengerDelete(const UINT pos)=0;
   virtual MTAPIRES  MessengerShift(const UINT pos,const int shift)=0;
   virtual UINT      MessengerTotal(void)=0;
   virtual MTAPIRES  MessengerNext(const UINT pos,IMTConMessenger* messenger)=0;
   virtual MTAPIRES  MessengerGet(LPCWSTR name,IMTConMessenger* messenger)=0;
   virtual MTAPIRES  MessengerSend(LPCWSTR destination,LPCWSTR group,LPCWSTR sender,LPCWSTR text)=0;
   virtual MTAPIRES  MessengerVerifyPhone(LPCWSTR phone_number)=0;
   virtual IMTConMessengerCountry* MessengerCountryCreate()=0;
   virtual IMTConMessengerGroup* MessengerGroupCreate()=0;
   //--- trade positions database
   virtual MTAPIRES  PositionSelectByGroup(LPCWSTR group,INT64 from,INT64 to,const IMTDatasetRequest *request,IMTDataset *dataset)=0;
   virtual MTAPIRES  PositionSelectByLogins(const UINT64 *logins,UINT logins_total,INT64 from,INT64 to,const IMTDatasetRequest *request,IMTDataset *dataset)=0;
   virtual MTAPIRES  PositionGetByGroup(LPCWSTR group,IMTPositionArray *positions)=0;
   virtual MTAPIRES  PositionGetByLogins(const UINT64 *logins,UINT logins_total,IMTPositionArray *positions)=0;
   virtual MTAPIRES  PositionGetByTickets(const UINT64 *tickets,UINT tickets_total,IMTPositionArray *positions)=0;
   virtual MTAPIRES  PositionSplit(const UINT64* tickets,const UINT tickets_total,const double *adjustments,const UINT new_shares,const UINT old_shares,const UINT round_rule_price,const UINT round_rule_volume,const UINT flags,MTAPIRES* results)=0;
   virtual MTAPIRES  PositionReserved2(void)=0;
   virtual MTAPIRES  PositionReserved3(void)=0;
   virtual MTAPIRES  PositionReserved4(void)=0;
   //--- dataset management
   virtual IMTDatasetRequest* DatasetRequestCreate(void)=0;
   virtual IMTDataset* DatasetCreate(void)=0;
   virtual MTAPIRES  DatasetReserved1(void)=0;
   virtual MTAPIRES  DatasetReserved2(void)=0;
   virtual MTAPIRES  DatasetReserved3(void)=0;
   virtual MTAPIRES  DatasetReserved4(void)=0;
   //--- orders
   virtual MTAPIRES  OrderGetByTickets(const UINT64 *tickets,UINT tickets_total,IMTOrderArray *orders)=0;
   virtual MTAPIRES  OrderReserved1(void)=0;
   virtual MTAPIRES  OrderReserved2(void)=0;
   virtual MTAPIRES  OrderReserved3(void)=0;
   virtual MTAPIRES  OrderReserved4(void)=0;
   //--- history
   virtual MTAPIRES  HistoryGetByTickets(const UINT64 *tickets,UINT tickets_total,IMTOrderArray *orders)=0;
   virtual MTAPIRES  HistoryReserved1(void)=0;
   virtual MTAPIRES  HistoryReserved2(void)=0;
   virtual MTAPIRES  HistoryReserved3(void)=0;
   virtual MTAPIRES  HistoryReserved4(void)=0;
   //--- dealing
   virtual MTAPIRES  DealGetByTickets(const UINT64 *tickets,UINT tickets_total,IMTDealArray *deals)=0;
   virtual MTAPIRES  DealReserved1(void)=0;
   virtual MTAPIRES  DealReserved2(void)=0;
   virtual MTAPIRES  DealReserved3(void)=0;
   virtual MTAPIRES  DealReserved4(void)=0;
   //--- client management
   virtual IMTClient *ClientCreate(void)=0;
   virtual IMTClientArray *ClientCreateArray(void)=0;
   virtual MTAPIRES  ClientSubscribe(IMTClientSink* sink)=0;
   virtual MTAPIRES  ClientUnsubscribe(IMTClientSink* sink)=0;
   virtual MTAPIRES  ClientAdd(IMTClient *client,const UINT64 author)=0;
   virtual MTAPIRES  ClientUpdate(IMTClient *client,const UINT64 author)=0;
   virtual MTAPIRES  ClientDelete(const UINT64 client_id,const UINT64 author)=0;
   virtual MTAPIRES  ClientGet(const UINT64 client_id,IMTClient *client)=0;
   virtual MTAPIRES  ClientGetHistory(const UINT64 client_id,const UINT64 author,const INT64 from,const INT64 to,IMTClientArray *history)=0;
   virtual MTAPIRES  ClientIdsAll(UINT64*& ids,UINT& ids_total)=0;
   virtual MTAPIRES  ClientIdsByGroup(const LPCWSTR groups,UINT64*& ids,UINT& ids_total)=0;
   virtual MTAPIRES  ClientIdsByManager(const UINT64 manager,UINT64*& ids,UINT& ids_total)=0;
   virtual MTAPIRES  ClientUserAdd(const UINT64 client_id,const UINT64 login)=0;
   virtual MTAPIRES  ClientUserDelete(const UINT64 client_id,const UINT64 login)=0;
   virtual MTAPIRES  ClientUserLogins(const UINT64 client_id,UINT64*& logins,UINT& logins_total)=0;
   virtual MTAPIRES  ClientReserved1(void)=0;
   virtual MTAPIRES  ClientReserved2(void)=0;
   virtual MTAPIRES  ClientReserved3(void)=0;
   virtual MTAPIRES  ClientReserved4(void)=0;
   //--- document management
   virtual IMTDocument *DocumentCreate(void)=0;
   virtual IMTDocumentArray *DocumentCreateArray(void)=0;
   virtual MTAPIRES  DocumentSubscribe(IMTDocumentSink* sink)=0;
   virtual MTAPIRES  DocumentUnsubscribe(IMTDocumentSink* sink)=0;
   virtual MTAPIRES  DocumentAdd(IMTDocument *document,const UINT64 author)=0;
   virtual MTAPIRES  DocumentUpdate(IMTDocument *document,const UINT64 author)=0;
   virtual MTAPIRES  DocumentDelete(const UINT64 document_id,const UINT64 author)=0;
   virtual MTAPIRES  DocumentGet(const UINT64 document_id,IMTDocument *document)=0;
   virtual MTAPIRES  DocumentGetByClient(const UINT64 client_id,const UINT position,const UINT total,IMTDocumentArray *documents)=0;
   virtual MTAPIRES  DocumentGetHistory(const UINT64 document_id,const UINT64 author,const INT64 from,const INT64 to,IMTDocumentArray *history)=0;
   virtual MTAPIRES  DocumentReserved1(void)=0;
   virtual MTAPIRES  DocumentReserved2(void)=0;
   virtual MTAPIRES  DocumentReserved3(void)=0;
   virtual MTAPIRES  DocumentReserved4(void)=0;
   //--- comment management
   virtual IMTComment *CommentCreate(void)=0;
   virtual IMTCommentArray *CommentCreateArray(void)=0;
   virtual MTAPIRES  CommentSubscribe(IMTCommentSink* sink)=0;
   virtual MTAPIRES  CommentUnsubscribe(IMTCommentSink* sink)=0;
   virtual MTAPIRES  CommentAdd(IMTComment *comment,const UINT64 author)=0;
   virtual MTAPIRES  CommentUpdate(IMTComment *comment,const UINT64 author)=0;
   virtual MTAPIRES  CommentDelete(const UINT64 comment_id,const UINT64 author)=0;
   virtual MTAPIRES  CommentGet(const UINT64 comment_id,IMTComment *comment)=0;
   virtual MTAPIRES  CommentGetByClient(const UINT64 client_id,const UINT position,const UINT total,IMTCommentArray *comments)=0;
   virtual MTAPIRES  CommentGetByDocument(const UINT64 document_id,const UINT position,const UINT total,IMTCommentArray *comments)=0;
   virtual MTAPIRES  CommentReserved1(void)=0;
   virtual MTAPIRES  CommentReserved2(void)=0;
   virtual MTAPIRES  CommentReserved3(void)=0;
   virtual MTAPIRES  CommentReserved4(void)=0;
   //--- attachment management
   virtual IMTAttachment* AttachmentCreate(void)=0;
   virtual MTAPIRES  AttachmentAdd(IMTAttachment *attachment,const UINT64 author)=0;
   virtual MTAPIRES  AttachmentGet(const UINT64 attachment_id,IMTAttachment *attachment)=0;
   virtual MTAPIRES  AttachmentReserved1(void)=0;
   virtual MTAPIRES  AttachmentReserved2(void)=0;
   virtual MTAPIRES  AttachmentReserved3(void)=0;
   virtual MTAPIRES  AttachmentReserved4(void)=0;
   //--- TLS Certificates
   virtual MTAPIRES  TLSCertificateUpdate(const void* pfx_certificate,const UINT pfx_certificate_size,LPCWSTR password)=0;
   virtual MTAPIRES  TLSCertificateDelete(const UINT pos)=0;
   virtual MTAPIRES  TLSCertificateShift(const UINT pos,const int shift)=0;
   virtual UINT      TLSCertificateTotal(void)=0;
   virtual MTAPIRES  TLSCertificateNext(const UINT pos,MTAPISTR& name,MTAPISTR& thumbprint)=0;
   virtual void*     TLSCertificatePfx(const UINT pos,UINT& pfx_certificate_size)=0;
   virtual MTAPIRES  TLSCertificateReserved1(void)=0;
   virtual MTAPIRES  TLSCertificateReserved2(void)=0;
   virtual MTAPIRES  TLSCertificateReserved3(void)=0;
   virtual MTAPIRES  TLSCertificateReserved4(void)=0;
   //--- automation configuration
   virtual IMTConAutomation* AutomationCreate()=0;
   virtual IMTConAutoCondition* AutomationConditionCreate()=0;
   virtual IMTConAutoAction* AutomationActionCreate()=0;
   virtual IMTConAutoParam* AutomationParamCreate()=0;
   virtual MTAPIRES  AutomationSubscribe(IMTConAutomationSink* sink)=0;
   virtual MTAPIRES  AutomationUnsubscribe(IMTConAutomationSink* sink)=0;
   virtual MTAPIRES  AutomationAdd(IMTConAutomation* config)=0;
   virtual MTAPIRES  AutomationDelete(LPCWSTR name)=0;
   virtual MTAPIRES  AutomationDelete(const UINT pos)=0;
   virtual MTAPIRES  AutomationShift(const UINT pos,const int shift)=0;
   virtual UINT      AutomationTotal(void)=0;
   virtual MTAPIRES  AutomationNext(const UINT pos,IMTConAutomation* config)=0;
   virtual MTAPIRES  AutomationGet(LPCWSTR name,IMTConAutomation* config)=0;
   virtual MTAPIRES  AutomationTrigger(LPCWSTR name,const IMTUser* user,const IMTAccount* account,const IMTDeal* deal,const IMTOrder* order,const IMTPosition* position)=0;
   virtual MTAPIRES  AutomationReserved2(void)=0;
   virtual MTAPIRES  AutomationReserved3(void)=0;
   virtual MTAPIRES  AutomationReserved4(void)=0;
   //--- subscription configuration
   virtual IMTConSubscription* SubscriptionCfgCreate()=0;
   virtual IMTConSubscriptionSymbol* SubscriptionCfgSymbolCreate()=0;
   virtual IMTConSubscriptionNews* SubscriptionCfgNewsCreate()=0;
   virtual MTAPIRES  SubscriptionCfgSubscribe(IMTConSubscriptionSink* sink)=0;
   virtual MTAPIRES  SubscriptionCfgUnsubscribe(IMTConSubscriptionSink* sink)=0;
   virtual MTAPIRES  SubscriptionCfgAdd(IMTConSubscription* config)=0;
   virtual MTAPIRES  SubscriptionCfgDelete(LPCWSTR name)=0;
   virtual MTAPIRES  SubscriptionCfgDelete(const UINT pos)=0;
   virtual MTAPIRES  SubscriptionCfgDeleteByID(const UINT64 id)=0;
   virtual MTAPIRES  SubscriptionCfgShift(const UINT pos,const int shift)=0;
   virtual UINT      SubscriptionCfgTotal(void)=0;
   virtual MTAPIRES  SubscriptionCfgNext(const UINT pos,IMTConSubscription* config)=0;
   virtual MTAPIRES  SubscriptionCfgGet(LPCWSTR name,IMTConSubscription* config)=0;
   virtual MTAPIRES  SubscriptionCfgGetByID(const UINT64 id,IMTConSubscription* config)=0;
   virtual MTAPIRES  SubscriptionCfgReserved1(void)=0;
   virtual MTAPIRES  SubscriptionCfgReserved2(void)=0;
   virtual MTAPIRES  SubscriptionCfgReserved3(void)=0;
   virtual MTAPIRES  SubscriptionCfgReserved4(void)=0;
   //--- subscription records
   virtual IMTSubscription* SubscriptionCreate()=0;
   virtual IMTSubscriptionArray* SubscriptionCreateArray(void)=0;
   virtual MTAPIRES  SubscriptionSubscribe(IMTSubscriptionSink* sink)=0;
   virtual MTAPIRES  SubscriptionUnsubscribe(IMTSubscriptionSink* sink)=0;
   virtual MTAPIRES  SubscriptionJoin(const UINT64 manager,const UINT64 login,const UINT64 subscription,IMTSubscription* record,IMTSubscriptionHistory* history)=0;
   virtual MTAPIRES  SubscriptionCancel(const UINT64 manager,const UINT64 login,const UINT64 subscription,IMTSubscription* record,IMTSubscriptionHistory* history)=0;
   virtual bool      SubscriptionExist(const UINT64 login,const UINT64 subscription)=0;
   virtual MTAPIRES  SubscriptionAdd(IMTSubscription* record)=0;
   virtual MTAPIRES  SubscriptionUpdate(IMTSubscription* record)=0;
   virtual MTAPIRES  SubscriptionDelete(const UINT64 id)=0;
   virtual MTAPIRES  SubscriptionGet(const UINT64 login,IMTSubscriptionArray* records)=0;
   virtual MTAPIRES  SubscriptionGetBySubscription(const UINT64 login,const UINT64 subscription,IMTSubscription* record)=0;
   virtual MTAPIRES  SubscriptionGetByID(const UINT64 id,IMTSubscription* record)=0;
   virtual MTAPIRES  SubscriptionGetByLogins(const UINT64 *logins,UINT logins_total,IMTSubscriptionArray* records)=0;
   virtual MTAPIRES  SubscriptionReserved1(void)=0;
   virtual MTAPIRES  SubscriptionReserved2(void)=0;
   virtual MTAPIRES  SubscriptionReserved3(void)=0;
   virtual MTAPIRES  SubscriptionReserved4(void)=0;
   //--- subscription history records
   virtual IMTSubscriptionHistory* SubscriptionHistoryCreate()=0;
   virtual IMTSubscriptionHistoryArray* SubscriptionHistoryCreateArray(void)=0;
   virtual MTAPIRES  SubscriptionHistorySubscribe(IMTSubscriptionHistorySink* sink)=0;
   virtual MTAPIRES  SubscriptionHistoryUnsubscribe(IMTSubscriptionHistorySink* sink)=0;
   virtual MTAPIRES  SubscriptionHistoryAdd(IMTSubscriptionHistory* record)=0;
   virtual MTAPIRES  SubscriptionHistoryUpdate(IMTSubscriptionHistory* record)=0;
   virtual MTAPIRES  SubscriptionHistoryDelete(const UINT64 id)=0;
   virtual MTAPIRES  SubscriptionHistoryGet(const INT64 from,const INT64 to,const UINT64 login,IMTSubscriptionHistoryArray* records)=0;
   virtual MTAPIRES  SubscriptionHistoryGetByID(const UINT64 id,IMTSubscriptionHistory* record)=0;
   virtual MTAPIRES  SubscriptionHistoryGetByLogins(const INT64 from,const INT64 to,const UINT64 *logins,UINT logins_total,IMTSubscriptionHistoryArray* records)=0;
   virtual MTAPIRES  SubscriptionHistoryReserved1(void)=0;
   virtual MTAPIRES  SubscriptionHistoryReserved2(void)=0;
   virtual MTAPIRES  SubscriptionHistoryReserved3(void)=0;
   virtual MTAPIRES  SubscriptionHistoryReserved4(void)=0;
   //--- geodata
   virtual MTAPIRES  GeoResolveAny(LPCWSTR ip_list,LPWSTR result,const UINT result_len,const UINT flags)=0;
   virtual MTAPIRES  GeoResolveIPv4(const ULONG ip,LPWSTR result,const UINT result_len,const UINT flags) =0;
   virtual MTAPIRES  GeoResolveIPv4Bulk(const ULONG *ip_list,const UINT ip_list_len,LPWSTR result,const UINT result_len,const UINT flags)=0;
   virtual MTAPIRES  GeoResolveIPv6(const IN6_ADDR &ip,LPWSTR result,const UINT result_len,const UINT flags)=0;
   virtual MTAPIRES  GeoResolveIPv6Bulk(const IN6_ADDR *ip_list,const UINT ip_list_len,LPWSTR result,const UINT result_len,const UINT flags)=0;
   virtual MTAPIRES  GeoResolveReserved1(void)=0;
   virtual MTAPIRES  GeoResolveReserved2(void)=0;
   virtual MTAPIRES  GeoResolveReserved3(void)=0;
   virtual MTAPIRES  GeoResolveReserved4(void)=0;
   //--- VPS config
   virtual IMTConVPS* VPSCreate(void)=0;
   virtual IMTConVPSGroup* VPSCreateGroup(void)=0;
   virtual MTAPIRES  VPSSubscribe(IMTConVPSSink* sink)=0;
   virtual MTAPIRES  VPSUnsubscribe(IMTConVPSSink* sink)=0;
   virtual MTAPIRES  VPSGet(IMTConVPS* config)=0;
   virtual MTAPIRES  VPSSet(const IMTConVPS* config)=0;
   virtual MTAPIRES  VPSReserved1(void)=0;
   virtual MTAPIRES  VPSReserved2(void)=0;
   virtual MTAPIRES  VPSReserved3(void)=0;
   virtual MTAPIRES  VPSReserved4(void)=0;
   //--- KYC config
   virtual IMTConKYC* KYCCreate(void)=0;
   virtual IMTConKYCCountry* KYCCountryCreate()=0;
   virtual IMTConKYCGroup* KYCGroupCreate()=0;
   virtual MTAPIRES  KYCSubscribe(IMTConKYCSink* sink)=0;
   virtual MTAPIRES  KYCUnsubscribe(IMTConKYCSink* sink)=0;
   virtual MTAPIRES  KYCAdd(IMTConKYC* config)=0;
   virtual MTAPIRES  KYCDelete(LPCWSTR name)=0;
   virtual MTAPIRES  KYCDelete(const UINT pos)=0;
   virtual MTAPIRES  KYCShift(const UINT pos,const int shift)=0;
   virtual UINT      KYCTotal(void)=0;
   virtual MTAPIRES  KYCNext(const UINT pos,IMTConKYC* kyc)=0;
   virtual MTAPIRES  KYCGet(LPCWSTR name,IMTConKYC* kyc)=0;
   virtual MTAPIRES  KYCStart(const UINT64 client_id)=0;
   virtual MTAPIRES  KYCReserved1(void)=0;
   virtual MTAPIRES  KYCReserved2(void)=0;
   virtual MTAPIRES  KYCReserved3(void)=0;
   virtual MTAPIRES  KYCReserved4(void)=0;
  };

  class IMTServerPlugin
  {
public:
   virtual void      Release(void)=0;
   //--- plugin start & stop notification
   virtual MTAPIRES  Start(IMTServerAPI* server)=0;
   virtual MTAPIRES  Stop(void)=0;
  };
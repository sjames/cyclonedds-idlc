/****************************************************************

  Generated by Eclipse Cyclone DDS IDL to C Translator
  File name: test.c
  Source: test.idl
  Cyclone DDS: V0.7.0

*****************************************************************/
#include "test.h"


static const dds_key_descriptor_t TestData_Msg_keys[3] =
{
  { "short_field", 0 },
  { "long_field", 2 },
  { "string_field", 18 }
};

static const uint32_t TestData_Msg_ops [] =
{
  DDS_OP_ADR | DDS_OP_TYPE_2BY | DDS_OP_FLAG_SGN | DDS_OP_FLAG_KEY, offsetof (TestData_Msg, short_field),
  DDS_OP_ADR | DDS_OP_TYPE_4BY | DDS_OP_FLAG_SGN | DDS_OP_FLAG_KEY, offsetof (TestData_Msg, long_field),
  DDS_OP_ADR | DDS_OP_TYPE_2BY, offsetof (TestData_Msg, ushort_field),
  DDS_OP_ADR | DDS_OP_TYPE_4BY, offsetof (TestData_Msg, ulong_field),
  DDS_OP_ADR | DDS_OP_TYPE_4BY | DDS_OP_FLAG_FP, offsetof (TestData_Msg, float_field),
  DDS_OP_ADR | DDS_OP_TYPE_8BY | DDS_OP_FLAG_FP, offsetof (TestData_Msg, double_field),
  DDS_OP_ADR | DDS_OP_TYPE_1BY | DDS_OP_FLAG_SGN, offsetof (TestData_Msg, char_field),
  DDS_OP_ADR | DDS_OP_TYPE_BOO, offsetof (TestData_Msg, bool_field),
  DDS_OP_ADR | DDS_OP_TYPE_1BY, offsetof (TestData_Msg, octet_field),
  DDS_OP_ADR | DDS_OP_TYPE_STR | DDS_OP_FLAG_KEY, offsetof (TestData_Msg, string_field),
  DDS_OP_ADR | DDS_OP_TYPE_SEQ | DDS_OP_SUBTYPE_STR, offsetof (TestData_Msg, sequence_field),
  DDS_OP_ADR | DDS_OP_TYPE_ARR | DDS_OP_SUBTYPE_2BY | DDS_OP_FLAG_SGN, offsetof (TestData_Msg, array_field), 25,
  DDS_OP_ADR | DDS_OP_TYPE_ARR | DDS_OP_SUBTYPE_4BY | DDS_OP_FLAG_FP, offsetof (TestData_Msg, twod_array_field), 750,
  DDS_OP_RTS
};

const dds_topic_descriptor_t TestData_Msg_desc =
{
  sizeof (TestData_Msg),
  8u,
  DDS_TOPIC_NO_OPTIMIZE,
  3u,
  "TestData::Msg",
  TestData_Msg_keys,
  14,
  TestData_Msg_ops,
  "<MetaData version=\"1.0.0\"><Module name=\"TestData\"><Struct name=\"Msg\"><Member name=\"short_field\"><Short/></Member><Member name=\"long_field\"><Long/></Member><Member name=\"ushort_field\"><UShort/></Member><Member name=\"ulong_field\"><ULong/></Member><Member name=\"float_field\"><Float/></Member><Member name=\"double_field\"><Double/></Member><Member name=\"char_field\"><Char/></Member><Member name=\"bool_field\"><Boolean/></Member><Member name=\"octet_field\"><Octet/></Member><Member name=\"string_field\"><String/></Member><Member name=\"sequence_field\"><Sequence><String/></Sequence></Member><Member name=\"array_field\"><Array size=\"25\"><Short/></Array></Member><Member name=\"twod_array_field\"><Array size=\"25\"><Array size=\"30\"><Float/></Array></Array></Member></Struct></Module></MetaData>"
};





static const dds_key_descriptor_t TestData_Inner_TopicMsg_keys[1] =
{
  { "topicID", 4 }
};

static const uint32_t TestData_Inner_TopicMsg_ops [] =
{
  DDS_OP_ADR | DDS_OP_TYPE_4BY | DDS_OP_FLAG_SGN, offsetof (TestData_Inner_TopicMsg, inner_msg.userID),
  DDS_OP_ADR | DDS_OP_TYPE_STR, offsetof (TestData_Inner_TopicMsg, inner_msg.message),
  DDS_OP_ADR | DDS_OP_TYPE_STR | DDS_OP_FLAG_KEY, offsetof (TestData_Inner_TopicMsg, topicID),
  DDS_OP_ADR | DDS_OP_TYPE_STR, offsetof (TestData_Inner_TopicMsg, common.global_name),
  DDS_OP_ADR | DDS_OP_TYPE_BOO, offsetof (TestData_Inner_TopicMsg, common.enabled),
  DDS_OP_RTS
};

const dds_topic_descriptor_t TestData_Inner_TopicMsg_desc =
{
  sizeof (TestData_Inner_TopicMsg),
  sizeof (char *),
  DDS_TOPIC_NO_OPTIMIZE,
  1u,
  "TestData::Inner::TopicMsg",
  TestData_Inner_TopicMsg_keys,
  6,
  TestData_Inner_TopicMsg_ops,
  "<MetaData version=\"1.0.0\"><Module name=\"TestData\"><Struct name=\"CommonStruct\"><Member name=\"global_name\"><String/></Member><Member name=\"enabled\"><Boolean/></Member></Struct><Module name=\"Inner\"><Struct name=\"InnerMsg\"><Member name=\"userID\"><Long/></Member><Member name=\"message\"><String/></Member></Struct><Struct name=\"TopicMsg\"><Member name=\"inner_msg\"><Type name=\"InnerMsg\"/></Member><Member name=\"topicID\"><String/></Member><Member name=\"common\"><Type name=\"TestData::CommonStruct\"/></Member></Struct></Module></Module></MetaData>"
};